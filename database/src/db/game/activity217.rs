use anyhow::Result;
use sqlx::SqlitePool;

pub struct Activity217State {
    pub exp_episode_count: i32,
    pub coin_episode_count: i32,
    pub type_states: Vec<Activity217TypeState>,
}

pub struct Activity217TypeState {
    pub r#type: i32,
    pub daily_use_count: i32,
    pub total_use_count: i32,
}

pub async fn sync(
    pool: &SqlitePool,
    user_id: i64,
    activity_id: i32,
    tables: &config::GameDB,
) -> Result<()> {
    let now = common::time::ServerTime::now_ms();

    sqlx::query(
        "INSERT INTO user_activity217_state (user_id, activity_id, updated_at)
         VALUES (?, ?, ?)
         ON CONFLICT DO NOTHING",
    )
    .bind(user_id)
    .bind(activity_id)
    .bind(now)
    .execute(pool)
    .await?;

    for row in tables
        .activity217_control
        .iter()
        .filter(|row| row.activity_id == activity_id)
    {
        sqlx::query(
            "INSERT INTO user_activity217_type_state (user_id, activity_id, type, updated_at)
             VALUES (?, ?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(activity_id)
        .bind(row.r#type)
        .bind(now)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn get(pool: &SqlitePool, user_id: i64, activity_id: i32) -> Result<Activity217State> {
    let (exp_episode_count, coin_episode_count) = sqlx::query_as::<_, (i32, i32)>(
        "SELECT exp_episode_count, coin_episode_count
         FROM user_activity217_state
         WHERE user_id = ? AND activity_id = ?",
    )
    .bind(user_id)
    .bind(activity_id)
    .fetch_one(pool)
    .await?;

    let type_states = sqlx::query_as::<_, (i32, i32, i32)>(
        "SELECT type, daily_use_count, total_use_count
         FROM user_activity217_type_state
         WHERE user_id = ? AND activity_id = ?
         ORDER BY type",
    )
    .bind(user_id)
    .bind(activity_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(
        |(r#type, daily_use_count, total_use_count)| Activity217TypeState {
            r#type,
            daily_use_count,
            total_use_count,
        },
    )
    .collect();

    Ok(Activity217State {
        exp_episode_count,
        coin_episode_count,
        type_states,
    })
}
