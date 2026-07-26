use super::*;

pub async fn load_starter_guides(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
) -> sqlx::Result<()> {
    for guide in config::configs::get()
        .guide
        .iter()
        .filter(|guide| guide.is_online != 0 && guide.trigger == "PlayerLv#1")
    {
        sqlx::query(
            "INSERT INTO guide_progress (user_id, guide_id, step_id) VALUES (?, ?, 0)
             ON CONFLICT(user_id, guide_id) DO NOTHING",
        )
        .bind(user_id)
        .bind(guide.id)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}
