use crate::models::game::command_post::*;
use anyhow::Result;
use sqlx::SqlitePool;

pub async fn get_command_post_info(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<(
    UserCommandPostInfo,
    Vec<CommandPostEventInfo>,
    Vec<CommandPostTask>,
    Vec<CommandPostTask>,
    Vec<i32>,
    Vec<i32>,
)> {
    // Get main info
    let info = sqlx::query_as::<_, UserCommandPostInfo>(
        "SELECT * FROM user_command_post_info WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .unwrap_or(UserCommandPostInfo {
        user_id,
        paper: 0,
        catch_num: 0,
    });

    // Get events
    let events = get_command_post_events(pool, user_id).await?;

    // Get tasks
    let tasks = sqlx::query_as::<_, CommandPostTask>(
        "SELECT task_id, progress, has_finished, finish_count, task_type, expiry_time
         FROM user_command_post_tasks WHERE user_id = ? ORDER BY task_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    // Get catch tasks
    let catch_tasks = sqlx::query_as::<_, CommandPostTask>(
        "SELECT task_id, progress, has_finished, finish_count, task_type, expiry_time
         FROM user_command_post_catch_tasks WHERE user_id = ? ORDER BY task_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    // Get gain bonuses
    let gain_bonus = sqlx::query_scalar(
        "SELECT bonus_id FROM user_command_post_gain_bonus WHERE user_id = ? ORDER BY bonus_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let character_state = sqlx::query_scalar(
        "SELECT state_id FROM user_command_post_character_state WHERE user_id = ? ORDER BY state_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok((
        info,
        events,
        tasks,
        catch_tasks,
        gain_bonus,
        character_state,
    ))
}

pub async fn read_command_post_character(
    pool: &SqlitePool,
    user_id: i64,
    state_id: i32,
) -> Result<()> {
    sqlx::query(
        "INSERT OR IGNORE INTO user_command_post_character_state (user_id, state_id) VALUES (?, ?)",
    )
    .bind(user_id)
    .bind(state_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn read_command_post_event(pool: &SqlitePool, user_id: i64, event_id: i32) -> Result<()> {
    sqlx::query(
        "UPDATE user_command_post_events SET is_read = 1 WHERE user_id = ? AND event_id = ?",
    )
    .bind(user_id)
    .bind(event_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn claim_command_post_bonus(
    pool: &SqlitePool,
    user_id: i64,
    bonus_id: i32,
) -> Result<bool> {
    let result = sqlx::query(
        "INSERT OR IGNORE INTO user_command_post_gain_bonus (user_id, bonus_id) VALUES (?, ?)",
    )
    .bind(user_id)
    .bind(bonus_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() != 0)
}

pub async fn claim_command_post_bonus_in_transaction(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    user_id: i64,
    bonus_id: i32,
) -> Result<bool> {
    let result = sqlx::query(
        "INSERT OR IGNORE INTO user_command_post_gain_bonus (user_id, bonus_id) VALUES (?, ?)",
    )
    .bind(user_id)
    .bind(bonus_id)
    .execute(&mut **tx)
    .await?;

    Ok(result.rows_affected() != 0)
}

pub async fn compose_command_post_paper(pool: &SqlitePool, user_id: i64) -> Result<i32> {
    sqlx::query(
        "INSERT INTO user_command_post_info (user_id, paper, catch_num) VALUES (?, 1, 0)
         ON CONFLICT(user_id) DO UPDATE SET paper = paper + 1",
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    let paper = sqlx::query_scalar("SELECT paper FROM user_command_post_info WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(paper)
}

pub async fn dispatch_command_post_event(
    pool: &SqlitePool,
    user_id: i64,
    event_id: i32,
    hero_ids: &[i32],
    start_time: i64,
    end_time: i64,
) -> Result<CommandPostEventInfo> {
    sqlx::query(
        "INSERT INTO user_command_post_events
            (user_id, event_id, state, start_time, end_time, is_read)
         VALUES (?, ?, 0, ?, ?, 1)
         ON CONFLICT(user_id, event_id) DO UPDATE SET
            state = 0,
            start_time = excluded.start_time,
            end_time = excluded.end_time,
            is_read = 1",
    )
    .bind(user_id)
    .bind(event_id)
    .bind(start_time)
    .bind(end_time)
    .execute(pool)
    .await?;

    sqlx::query("DELETE FROM user_command_post_event_heroes WHERE user_id = ? AND event_id = ?")
        .bind(user_id)
        .bind(event_id)
        .execute(pool)
        .await?;

    for hero_id in hero_ids {
        sqlx::query(
            "INSERT OR IGNORE INTO user_command_post_event_heroes
                (user_id, event_id, hero_id) VALUES (?, ?, ?)",
        )
        .bind(user_id)
        .bind(event_id)
        .bind(hero_id)
        .execute(pool)
        .await?;
    }

    get_command_post_event(pool, user_id, event_id).await
}

pub async fn finish_command_post_event(
    pool: &SqlitePool,
    user_id: i64,
    event_id: i32,
    state: i32,
) -> Result<()> {
    sqlx::query(
        "UPDATE user_command_post_events
         SET state = ?, is_read = 1
         WHERE user_id = ? AND event_id = ?",
    )
    .bind(state)
    .bind(user_id)
    .bind(event_id)
    .execute(pool)
    .await?;

    Ok(())
}

async fn get_command_post_event(
    pool: &SqlitePool,
    user_id: i64,
    event_id: i32,
) -> Result<CommandPostEventInfo> {
    let (event_id, state, start_time, end_time, is_read): (i32, i32, i64, i64, bool) =
        sqlx::query_as(
            "SELECT event_id, state, start_time, end_time, is_read
             FROM user_command_post_events WHERE user_id = ? AND event_id = ?",
        )
        .bind(user_id)
        .bind(event_id)
        .fetch_one(pool)
        .await?;

    let hero_ids = sqlx::query_scalar(
        "SELECT hero_id FROM user_command_post_event_heroes
         WHERE user_id = ? AND event_id = ? ORDER BY hero_id",
    )
    .bind(user_id)
    .bind(event_id)
    .fetch_all(pool)
    .await?;

    Ok(CommandPostEventInfo {
        event_id,
        state,
        hero_ids,
        start_time: start_time as u64,
        end_time: end_time as u64,
        is_read,
    })
}

async fn get_command_post_events(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<CommandPostEventInfo>> {
    let event_data: Vec<(i32, i32, i64, i64, bool)> = sqlx::query_as(
        "SELECT event_id, state, start_time, end_time, is_read
         FROM user_command_post_events WHERE user_id = ? ORDER BY event_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut events = Vec::new();
    for (event_id, state, start_time, end_time, is_read) in event_data {
        let hero_ids = sqlx::query_scalar(
            "SELECT hero_id FROM user_command_post_event_heroes WHERE user_id = ? AND event_id = ? ORDER BY hero_id"
        )
        .bind(user_id)
        .bind(event_id)
        .fetch_all(pool)
        .await?;

        events.push(CommandPostEventInfo {
            event_id,
            state,
            hero_ids,
            start_time: start_time as u64,
            end_time: end_time as u64,
            is_read,
        });
    }

    Ok(events)
}
