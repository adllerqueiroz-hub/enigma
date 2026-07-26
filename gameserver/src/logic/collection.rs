use crate::error::AppError;
use database::db::game::antiques;
use sonettobuf::GetAntiqueInfoReply;
use sqlx::SqlitePool;

pub async fn antique_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetAntiqueInfoReply, AppError> {
    Ok(GetAntiqueInfoReply {
        antiques: antiques::get_user_antiques(db, player_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    })
}
