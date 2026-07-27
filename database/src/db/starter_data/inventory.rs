use super::*;

pub async fn load_starter_currencies(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
) -> sqlx::Result<()> {
    let now = common::time::ServerTime::now_ms();
    let level: i32 = sqlx::query_scalar("SELECT level FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&mut **tx)
        .await?;
    let power_limit = configs::get()
        .player_level(level)
        .ok_or_else(|| sqlx::Error::Protocol(format!("missing player level {level}")))?
        .max_auto_recover_power;

    for currency in configs::get().currency.iter() {
        let quantity = if currency.id == crate::db::game::currencies::POWER_CURRENCY_ID {
            power_limit
        } else {
            0
        };
        sqlx::query(
            "INSERT INTO currencies
                (user_id, currency_id, quantity, last_recover_time, expired_time)
             VALUES (?, ?, ?, ?, 0)",
        )
        .bind(user_id)
        .bind(currency.id)
        .bind(quantity)
        .bind(now)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

pub async fn load_starter_user_stats(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT INTO user_stats (user_id, first_charge, total_charge_amount, is_first_login, user_tag)
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(user_id)
    .bind(false)
    .bind(0)
    .bind(true)
    .bind("")
    .execute(&mut **tx)
    .await?;

    tracing::info!("Loaded starter user stats for user {}", user_id);

    Ok(())
}
