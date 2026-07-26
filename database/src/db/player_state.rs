use crate::models::game::player_state::PlayerStateRecord;
use sqlx::SqlitePool;

pub async fn load(pool: &SqlitePool, player_id: i64) -> sqlx::Result<Option<PlayerStateRecord>> {
    sqlx::query_as::<_, PlayerStateRecord>("SELECT * FROM player_state WHERE player_id = ?1")
        .bind(player_id)
        .fetch_optional(pool)
        .await
}

pub async fn save(pool: &SqlitePool, state: &PlayerStateRecord) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO player_state (
            player_id, initial_login_complete, last_login_timestamp,
            created_at, updated_at,
            last_state_push_sent_timestamp, last_activity_push_sent_timestamp,
            last_daily_reward_time, last_daily_reset_time,
            month_card_claimed, last_month_card_claim_timestamp,
            last_sign_in_day, last_sign_in_time,
            vip_level,
            last_energy_refill_time, last_weekly_reset_time, last_monthly_reset_time
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)
        ON CONFLICT(player_id) DO UPDATE SET
            initial_login_complete = excluded.initial_login_complete,
            last_login_timestamp = excluded.last_login_timestamp,
            created_at = excluded.created_at,
            updated_at = excluded.updated_at,
            last_state_push_sent_timestamp = excluded.last_state_push_sent_timestamp,
            last_activity_push_sent_timestamp = excluded.last_activity_push_sent_timestamp,
            last_daily_reward_time = excluded.last_daily_reward_time,
            last_daily_reset_time = excluded.last_daily_reset_time,
            month_card_claimed = excluded.month_card_claimed,
            last_month_card_claim_timestamp = excluded.last_month_card_claim_timestamp,
            last_sign_in_day = excluded.last_sign_in_day,
            last_sign_in_time = excluded.last_sign_in_time,
            vip_level = excluded.vip_level,
            last_energy_refill_time = excluded.last_energy_refill_time,
            last_weekly_reset_time = excluded.last_weekly_reset_time,
            last_monthly_reset_time = excluded.last_monthly_reset_time
        "#,
    )
    .bind(state.player_id)
    .bind(state.initial_login_complete)
    .bind(state.last_login_timestamp)
    .bind(state.created_at)
    .bind(state.updated_at)
    .bind(state.last_state_push_sent_timestamp)
    .bind(state.last_activity_push_sent_timestamp)
    .bind(state.last_daily_reward_time)
    .bind(state.last_daily_reset_time)
    .bind(state.month_card_claimed)
    .bind(state.last_month_card_claim_timestamp)
    .bind(state.last_sign_in_day)
    .bind(state.last_sign_in_time)
    .bind(state.vip_level)
    .bind(state.last_energy_refill_time)
    .bind(state.last_weekly_reset_time)
    .bind(state.last_monthly_reset_time)
    .execute(pool)
    .await?;

    Ok(())
}
