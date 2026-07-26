mod assist;
mod card;
mod cloth;
mod info;

pub use info::level_up_rewards;

#[derive(Clone, Copy, Debug)]
pub struct ProfileManager {
    player_id: i64,
}

impl ProfileManager {
    pub fn new(player_id: i64) -> Self {
        Self { player_id }
    }

    pub async fn record_login_day(
        self,
        db: &sqlx::SqlitePool,
    ) -> Result<(), crate::error::AppError> {
        database::db::game::player_infos::increment_total_login_days(db, self.player_id).await?;
        Ok(())
    }
}
