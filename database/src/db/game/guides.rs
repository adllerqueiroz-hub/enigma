use crate::models::game::guides::GuideProgress;
use sqlx::SqlitePool;

pub async fn get_all_guide_progress(
    pool: &SqlitePool,
    user_id: i64,
) -> sqlx::Result<Vec<GuideProgress>> {
    sqlx::query_as::<_, GuideProgress>(
        "SELECT user_id, guide_id, step_id FROM guide_progress WHERE user_id = ? ORDER BY guide_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_guide_progress(
    pool: &SqlitePool,
    user_id: i64,
    guide_id: i32,
) -> sqlx::Result<Option<GuideProgress>> {
    sqlx::query_as::<_, GuideProgress>(
        "SELECT user_id, guide_id, step_id FROM guide_progress WHERE user_id = ? AND guide_id = ?",
    )
    .bind(user_id)
    .bind(guide_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_guide_progress(
    pool: &SqlitePool,
    user_id: i64,
    guide_id: i32,
    step_id: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT INTO guide_progress (user_id, guide_id, step_id)
         VALUES (?, ?, ?)
         ON CONFLICT(user_id, guide_id) DO UPDATE SET step_id = excluded.step_id",
    )
    .bind(user_id)
    .bind(guide_id)
    .bind(step_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_guide_progress_in_transaction(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    user_id: i64,
    guide_id: i32,
    expected_step: Option<i32>,
    step_id: i32,
) -> sqlx::Result<bool> {
    let result = if let Some(expected_step) = expected_step {
        sqlx::query(
            "UPDATE guide_progress SET step_id = ?
             WHERE user_id = ? AND guide_id = ? AND step_id = ?",
        )
        .bind(step_id)
        .bind(user_id)
        .bind(guide_id)
        .bind(expected_step)
        .execute(&mut **tx)
        .await?
    } else {
        sqlx::query(
            "INSERT OR IGNORE INTO guide_progress (user_id, guide_id, step_id)
             VALUES (?, ?, ?)",
        )
        .bind(user_id)
        .bind(guide_id)
        .bind(step_id)
        .execute(&mut **tx)
        .await?
    };
    Ok(result.rows_affected() == 1)
}

pub async fn complete_guide(pool: &SqlitePool, user_id: i64, guide_id: i32) -> sqlx::Result<()> {
    update_guide_progress(pool, user_id, guide_id, -1).await
}

pub async fn complete_all_guides(
    pool: &SqlitePool,
    user_id: i64,
    guide_ids: impl IntoIterator<Item = i32>,
) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    for guide_id in guide_ids {
        sqlx::query(
            "INSERT INTO guide_progress (user_id, guide_id, step_id)
             VALUES (?, ?, -1)
             ON CONFLICT(user_id, guide_id) DO UPDATE SET step_id = -1",
        )
        .bind(user_id)
        .bind(guide_id)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await
}
