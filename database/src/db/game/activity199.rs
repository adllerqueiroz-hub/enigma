use anyhow::Result;
use sqlx::SqlitePool;

pub async fn sync(pool: &SqlitePool, user_id: i64, activity_id: i32) -> Result<()> {
    sqlx::query(
        "INSERT INTO user_activity199_state (user_id, activity_id, updated_at)
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

pub async fn get(pool: &SqlitePool, user_id: i64, activity_id: i32) -> Result<i32> {
    Ok(sqlx::query_scalar(
        "SELECT hero_id
         FROM user_activity199_state
         WHERE user_id = ? AND activity_id = ?",
    )
    .bind(user_id)
    .bind(activity_id)
    .fetch_one(pool)
    .await?)
}

pub async fn set_hero(
    pool: &SqlitePool,
    user_id: i64,
    activity_id: i32,
    hero_id: i32,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO user_activity199_state (user_id, activity_id, hero_id, updated_at)
         VALUES (?, ?, ?, ?)
         ON CONFLICT(user_id, activity_id) DO UPDATE SET
             hero_id = excluded.hero_id,
             updated_at = excluded.updated_at",
    )
    .bind(user_id)
    .bind(activity_id)
    .bind(hero_id)
    .bind(common::time::ServerTime::now_ms())
    .execute(pool)
    .await?;

    Ok(())
}
