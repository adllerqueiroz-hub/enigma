use crate::error::AppError;
use database::db::game::activity217;
use sonettobuf::{Act217TypeInfo, Get217InfosReply};
use sqlx::SqlitePool;

pub async fn act217_infos(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Get217InfosReply, AppError> {
    let tables = config::configs::get();
    let activity_id = activity_id
        .or_else(|| {
            tables
                .activity217_control
                .iter()
                .map(|row| row.activity_id)
                .min()
        })
        .ok_or(AppError::InvalidRequest)?;
    activity217::sync(db, player_id, activity_id, tables).await?;
    let state = activity217::get(db, player_id, activity_id).await?;

    Ok(Get217InfosReply {
        activity_id: Some(activity_id),
        exp_episode_count: Some(state.exp_episode_count.max(0) as u32),
        coin_episode_count: Some(state.coin_episode_count.max(0) as u32),
        type_infos: state
            .type_states
            .into_iter()
            .map(|state| Act217TypeInfo {
                r#type: Some(state.r#type),
                daily_use_count: Some(state.daily_use_count),
                total_use_count: Some(state.total_use_count),
            })
            .collect(),
    })
}
