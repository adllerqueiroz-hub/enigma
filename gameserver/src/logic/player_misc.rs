use crate::error::AppError;
use database::db::game::cloths;
use sonettobuf::{GetAssistBonusReply, GetClothInfoReply, PlayerClothInfo};
use sqlx::SqlitePool;

pub fn get_assist_bonus() -> GetAssistBonusReply {
    GetAssistBonusReply {
        assist_bonus: Some(0),
        has_receive_assist_bonus: Some(0),
    }
}

pub async fn get_cloth_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetClothInfoReply, AppError> {
    Ok(GetClothInfoReply {
        cloth_infos: Some(PlayerClothInfo {
            clothes: cloths::get_all(db, player_id).await?,
        }),
    })
}
