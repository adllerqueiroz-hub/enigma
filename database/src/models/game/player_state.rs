use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PlayerStateRecord {
    pub player_id: i64,

    pub initial_login_complete: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub last_login_timestamp: Option<i64>,

    pub last_state_push_sent_timestamp: Option<i64>,
    pub last_activity_push_sent_timestamp: Option<i64>,

    pub last_daily_reward_time: Option<i64>,
    pub last_daily_reset_time: Option<i64>,

    pub month_card_claimed: bool,
    pub last_month_card_claim_timestamp: Option<i64>,

    pub last_sign_in_day: i64,
    pub last_sign_in_time: Option<i64>,

    pub vip_level: i32,
    pub last_energy_refill_time: Option<i64>,
    pub last_weekly_reset_time: Option<i64>,
    pub last_monthly_reset_time: Option<i64>,
}
