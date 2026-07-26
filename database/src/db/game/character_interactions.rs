use crate::models::game::character_interactions::{CharacterInteraction, CharacterInteractionInfo};
use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Transaction};

pub async fn get_character_interactions(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<CharacterInteractionInfo>> {
    let interactions = sqlx::query_as::<_, CharacterInteraction>(
        "SELECT * FROM user_character_interactions WHERE user_id = ? ORDER BY interaction_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for interaction in interactions {
        let select_ids = sqlx::query_scalar(
            "SELECT select_id FROM user_character_interaction_selections WHERE user_id = ? AND interaction_id = ? ORDER BY select_id"
        )
        .bind(user_id)
        .bind(interaction.interaction_id)
        .fetch_all(pool)
        .await?;

        result.push(CharacterInteractionInfo {
            interaction_id: interaction.interaction_id,
            is_finished: interaction.is_finished,
            select_ids,
        });
    }

    Ok(result)
}

pub async fn get_interaction_count(pool: &SqlitePool, user_id: i64) -> Result<i32> {
    let count: Option<i32> = sqlx::query_scalar(
        "SELECT interaction_count FROM user_interaction_stats WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(count.unwrap_or(0))
}

pub async fn start_interaction(
    pool: &SqlitePool,
    user_id: i64,
    interaction_id: i32,
) -> Result<bool> {
    let now = common::time::ServerTime::now_ms();
    let mut tx = pool.begin().await?;
    let inserted = sqlx::query(
        r#"
        INSERT INTO user_character_interactions (user_id, interaction_id, is_finished, created_at, updated_at)
        VALUES (?, ?, 0, ?, ?)
        ON CONFLICT(user_id, interaction_id) DO NOTHING
        "#,
    )
    .bind(user_id)
    .bind(interaction_id)
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await?;
    if inserted.rows_affected() == 0 {
        return Ok(false);
    }
    sqlx::query(
        r#"
        INSERT INTO user_interaction_stats (user_id, interaction_count)
        VALUES (?, 1)
        ON CONFLICT(user_id) DO UPDATE SET interaction_count = interaction_count + 1
        "#,
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(true)
}

pub async fn complete_interaction(
    pool: &SqlitePool,
    user_id: i64,
    interaction_id: i32,
    select_ids: &[i32],
) -> Result<bool> {
    let mut tx = pool.begin().await?;
    let completed =
        complete_interaction_in_transaction(&mut tx, user_id, interaction_id, select_ids).await?;
    tx.commit().await?;
    Ok(completed)
}

pub async fn complete_interaction_in_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    interaction_id: i32,
    select_ids: &[i32],
) -> Result<bool> {
    let now = common::time::ServerTime::now_ms();
    let updated = sqlx::query(
        "UPDATE user_character_interactions
         SET is_finished = 1, updated_at = ?
         WHERE user_id = ? AND interaction_id = ? AND is_finished = 0",
    )
    .bind(now)
    .bind(user_id)
    .bind(interaction_id)
    .execute(&mut **tx)
    .await?;
    if updated.rows_affected() == 0 {
        return Ok(false);
    }
    for select_id in select_ids {
        sqlx::query(
            "INSERT INTO user_character_interaction_selections
                 (user_id, interaction_id, select_id)
             VALUES (?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(interaction_id)
        .bind(select_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(true)
}
