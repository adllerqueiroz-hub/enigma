use crate::error::AppError;
use database::db::game::{player_infos, user_stats};
use sonettobuf::{StatInfoPush, UpdateClientStatBaseInfoReply};
use sqlx::SqlitePool;

pub async fn base_info(db: &SqlitePool, player_id: i64) -> Result<StatInfoPush, AppError> {
    let stats = user_stats::get_user_stats(db, player_id)
        .await?
        .ok_or_else(|| AppError::Custom("User stats not found".to_string()))?;

    let is_first_login = stats.is_first_login;
    let mut push: StatInfoPush = stats.into();
    if is_first_login {
        push.player_info = player_infos::get_player_info_data(db, player_id)
            .await?
            .map(Into::into);
    }
    user_stats::set_not_first_login(db, player_id).await?;
    Ok(push)
}

pub async fn update_base_info(
    _db: &SqlitePool,
    _player_id: i64,
) -> Result<UpdateClientStatBaseInfoReply, AppError> {
    Ok(UpdateClientStatBaseInfoReply::default())
}

#[cfg(test)]
mod test;
