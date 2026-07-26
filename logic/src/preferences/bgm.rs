use super::PreferenceManager;
use crate::error::AppError;
use database::db::game::bgm;
use sonettobuf::{GetBgmInfoReply, ReadBgmReply, SetFavoriteBgmReply, SetUseBgmReply};
use sqlx::SqlitePool;

impl PreferenceManager {
    pub async fn bgm_info(&self, db: &SqlitePool) -> Result<GetBgmInfoReply, AppError> {
        let (bgm_infos, use_bgm_id) = bgm::load_user_bgm(db, self.player_id).await?;
        Ok(GetBgmInfoReply {
            bgm_infos,
            use_bgm_id,
        })
    }

    pub async fn set_use_bgm(
        &self,
        db: &SqlitePool,
        bgm_id: i32,
    ) -> Result<SetUseBgmReply, AppError> {
        bgm::set_active_bgm(db, self.player_id, bgm_id).await?;
        Ok(SetUseBgmReply {
            bgm_id: Some(bgm_id),
        })
    }

    pub async fn read_bgm(&self, db: &SqlitePool, bgm_id: i32) -> Result<ReadBgmReply, AppError> {
        bgm::mark_bgm_read(db, self.player_id, bgm_id).await?;
        Ok(ReadBgmReply {
            bgm_id: Some(bgm_id),
        })
    }

    pub async fn set_favorite_bgm(
        &self,
        db: &SqlitePool,
        bgm_id: i32,
        favorite: bool,
    ) -> Result<SetFavoriteBgmReply, AppError> {
        bgm::set_bgm_favorite(db, self.player_id, bgm_id, favorite).await?;
        Ok(SetFavoriteBgmReply {
            bgm_id: Some(bgm_id),
            favorite: Some(favorite),
        })
    }
}
