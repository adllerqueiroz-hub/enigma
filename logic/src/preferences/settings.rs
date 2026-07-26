use super::PreferenceManager;
use crate::error::AppError;
use database::db::game::settings;
use sonettobuf::{GetSettingInfosReply, UpdateSettingInfoReply};
use sqlx::SqlitePool;

impl PreferenceManager {
    pub async fn setting_infos(&self, db: &SqlitePool) -> Result<GetSettingInfosReply, AppError> {
        Ok(GetSettingInfosReply {
            infos: settings::get_setting_infos(db, self.player_id).await?,
        })
    }

    pub async fn update_setting(
        &self,
        db: &SqlitePool,
        r#type: i32,
        param: String,
    ) -> Result<UpdateSettingInfoReply, AppError> {
        Ok(settings::update_setting_info(db, self.player_id, r#type, param).await?)
    }
}
