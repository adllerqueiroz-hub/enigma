use super::*;

pub async fn load_player_state(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let now = common::time::ServerTime::now_ms();
    sqlx::query(
        "INSERT INTO player_state (
            player_id, created_at, updated_at, last_daily_reset_time,
            last_sign_in_day, last_weekly_reset_time, last_monthly_reset_time
        ) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(now)
    .bind(now)
    .bind(now)
    .bind(common::time::ServerTime::server_day(now))
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn load_player_info(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let now = common::time::ServerTime::now_ms();
    let portrait = crate::db::game::player_infos::default_portrait_id()
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;

    // Create player_info record
    sqlx::query(
        "INSERT INTO player_info (
            player_id, signature, birthday, portrait, show_achievement, bg,
            total_login_days, last_episode_id, last_logout_time,
            hero_rare_nn_count, hero_rare_n_count, hero_rare_r_count,
            hero_rare_sr_count, hero_rare_ssr_count,
            created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
    )
    .bind(uid)
    .bind("") // signature
    .bind("")
    .bind(portrait)
    .bind("") // show achievement
    .bind(0) // bg
    .bind(0) // total_login_days
    .bind(0)
    .bind(None::<i64>)
    .bind(0) //hero_rare_nn_count
    .bind(0) //hero_rare_n_count
    .bind(0) //hero_rare_r_count
    .bind(0) // hero_rare_sr_count
    .bind(0) // hero_rare_ssr_count
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    sqlx::query(
        "INSERT INTO items (
            user_id, item_id, quantity, last_use_time, last_update_time, total_gain_count
        ) VALUES (?1, ?2, 1, NULL, ?3, 1)
        ON CONFLICT(user_id, item_id) DO NOTHING",
    )
    .bind(uid)
    .bind(portrait)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn load_open_infos(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let now = common::time::ServerTime::now_ms();
    for open in config::configs::get().open.iter() {
        sqlx::query(
            "INSERT INTO user_open_infos (user_id, open_id, is_open, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)
             ON CONFLICT(user_id, open_id) DO NOTHING",
        )
        .bind(uid)
        .bind(open.id)
        .bind(crate::db::game::open_infos::initial_state(open))
        .bind(now)
        .bind(now)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}
