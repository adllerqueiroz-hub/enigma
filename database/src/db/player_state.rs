use crate::models::game::player_state::PlayerStateRecord;
use sqlx::SqlitePool;

pub async fn load(pool: &SqlitePool, player_id: i64) -> sqlx::Result<Option<PlayerStateRecord>> {
    sqlx::query_as::<_, PlayerStateRecord>("SELECT * FROM player_state WHERE player_id = ?1")
        .bind(player_id)
        .fetch_optional(pool)
        .await
}

pub async fn save(pool: &SqlitePool, state: &PlayerStateRecord) -> sqlx::Result<()> {
    let updated = sqlx::query(
        r#"
        UPDATE player_state SET
            initial_login_complete = ?2,
            last_login_timestamp = ?3,
            created_at = ?4,
            updated_at = ?5,
            last_state_push_sent_timestamp = ?6,
            last_activity_push_sent_timestamp = ?7,
            last_daily_reward_time = ?8,
            last_daily_reset_time = ?9,
            month_card_claimed = ?10,
            last_month_card_claim_timestamp = ?11,
            last_sign_in_day = ?12,
            last_sign_in_time = ?13,
            vip_level = ?14,
            last_energy_refill_time = ?15,
            last_weekly_reset_time = ?16,
            last_monthly_reset_time = ?17
        WHERE player_id = ?1
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
    .await?
    .rows_affected();

    (updated != 0).then_some(()).ok_or(sqlx::Error::RowNotFound)
}
