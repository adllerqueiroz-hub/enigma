use anyhow::Result;
use sqlx::SqlitePool;

pub struct Activity225State {
    pub last_red_envelope_rain_id: i32,
    pub question_id: i32,
    pub rock_paper_scissors_daily_count: i32,
}

pub async fn sync(
    pool: &SqlitePool,
    user_id: i64,
    activity_id: i32,
    question_id: i32,
) -> Result<()> {
    let now = common::time::ServerTime::now_ms();
    let server_day = common::time::ServerTime::server_day(now);

    sqlx::query(
        "INSERT INTO user_activity225_state (
             user_id, activity_id, question_id, daily_reset_day, updated_at
         )
         VALUES (?, ?, ?, ?, ?)
         ON CONFLICT DO NOTHING",
    )
    .bind(user_id)
    .bind(activity_id)
    .bind(question_id)
    .bind(server_day)
    .bind(now)
    .execute(pool)
    .await?;

    sqlx::query(
        "UPDATE user_activity225_state
         SET rock_paper_scissors_daily_count = 0,
             daily_reset_day = ?,
             updated_at = ?
         WHERE user_id = ? AND activity_id = ? AND daily_reset_day != ?",
    )
    .bind(server_day)
    .bind(now)
    .bind(user_id)
    .bind(activity_id)
    .bind(server_day)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get(pool: &SqlitePool, user_id: i64, activity_id: i32) -> Result<Activity225State> {
    let (last_red_envelope_rain_id, question_id, rock_paper_scissors_daily_count) =
        sqlx::query_as::<_, (i32, i32, i32)>(
            "SELECT last_red_envelope_rain_id, question_id, rock_paper_scissors_daily_count
             FROM user_activity225_state
             WHERE user_id = ? AND activity_id = ?",
        )
        .bind(user_id)
        .bind(activity_id)
        .fetch_one(pool)
        .await?;

    Ok(Activity225State {
        last_red_envelope_rain_id,
        question_id,
        rock_paper_scissors_daily_count,
    })
}
