use sqlx::{Sqlite, Transaction};

pub async fn load_starter_summon(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT OR IGNORE INTO user_summon_stats
             (user_id, is_show_new_summon)
         VALUES (?, 1)",
    )
    .bind(user_id)
    .execute(&mut **tx)
    .await?;
    Ok(())
}
