use anyhow::Result;
use sqlx::SqlitePool;

pub struct Activity218State {
    pub finish_game_count: i32,
    pub total_coin_num: i32,
    pub accepted_reward_id: i32,
    pub game_record: String,
}

pub async fn sync(pool: &SqlitePool, user_id: i64, activity_id: i32) -> Result<()> {
    sqlx::query(
        "INSERT INTO user_activity218_state (user_id, activity_id, updated_at)
         VALUES (?, ?, ?)
         ON CONFLICT DO NOTHING",
    )
    .bind(user_id)
    .bind(activity_id)
    .bind(common::time::ServerTime::now_ms())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn finish_game(
    pool: &SqlitePool,
    user_id: i64,
    activity_id: i32,
    points: i32,
    max_coin: i32,
    game_record: &str,
) -> Result<Activity218State> {
    let now = common::time::ServerTime::now_ms();

    sqlx::query(
        "INSERT INTO user_activity218_state (
             user_id, activity_id, finish_game_count, total_coin_num, game_record, updated_at
         )
         VALUES (?, ?, 1, min(?, ?), ?, ?)
         ON CONFLICT(user_id, activity_id) DO UPDATE SET
             finish_game_count = finish_game_count + 1,
             total_coin_num = min(total_coin_num + excluded.total_coin_num, ?),
             game_record = excluded.game_record,
             updated_at = excluded.updated_at",
    )
    .bind(user_id)
    .bind(activity_id)
    .bind(points.max(0))
    .bind(max_coin.max(0))
    .bind(game_record)
    .bind(now)
    .bind(max_coin.max(0))
    .execute(pool)
    .await?;

    get(pool, user_id, activity_id).await
}

pub async fn accept_reward(
    pool: &SqlitePool,
    user_id: i64,
    activity_id: i32,
    accepted_reward_id: i32,
) -> Result<Activity218State> {
    sqlx::query(
        "UPDATE user_activity218_state
         SET accepted_reward_id = ?, updated_at = ?
         WHERE user_id = ? AND activity_id = ?",
    )
    .bind(accepted_reward_id)
    .bind(common::time::ServerTime::now_ms())
    .bind(user_id)
    .bind(activity_id)
    .execute(pool)
    .await?;

    get(pool, user_id, activity_id).await
}

pub async fn get(pool: &SqlitePool, user_id: i64, activity_id: i32) -> Result<Activity218State> {
    let (finish_game_count, total_coin_num, accepted_reward_id, game_record) =
        sqlx::query_as::<_, (i32, i32, i32, String)>(
            "SELECT finish_game_count, total_coin_num, accepted_reward_id, game_record
             FROM user_activity218_state
             WHERE user_id = ? AND activity_id = ?",
        )
        .bind(user_id)
        .bind(activity_id)
        .fetch_one(pool)
        .await?;

    Ok(Activity218State {
        finish_game_count,
        total_coin_num,
        accepted_reward_id,
        game_record,
    })
}
