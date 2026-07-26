use crate::error::AppError;
use database::db::game::{explore, weekwalk, weekwalk_v2};
use sonettobuf::{
    GetExploreSimpleInfoReply, GetWeekwalkInfoReply, WeekwalkInfo, WeekwalkVer2GetInfoReply,
};
use sqlx::SqlitePool;

pub async fn explore_simple_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetExploreSimpleInfoReply, AppError> {
    let (info, chapters, maps, unlock_map_ids) = explore::get_explore_info(db, player_id).await?;

    Ok(GetExploreSimpleInfoReply {
        last_map_id: Some(info.last_map_id),
        chapter_simple: chapters.into_iter().map(Into::into).collect(),
        map_simple: maps.into_iter().map(Into::into).collect(),
        unlock_map_ids,
        is_show_bag: Some(info.is_show_bag),
    })
}

pub async fn weekwalk_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetWeekwalkInfoReply, AppError> {
    let (info, map_info) = weekwalk::get_weekwalk_info(db, player_id).await?;

    Ok(GetWeekwalkInfoReply {
        info: Some(WeekwalkInfo {
            time: Some(info.time),
            end_time: Some(info.end_time),
            map_info,
            max_layer: Some(info.max_layer),
            issue_id: Some(info.issue_id),
            is_pop_deep_rule: Some(info.is_pop_deep_rule),
            is_open_deep: Some(info.is_open_deep),
            is_pop_shallow_settle: Some(info.is_pop_shallow_settle),
            is_pop_deep_settle: Some(info.is_pop_deep_settle),
            deep_progress: Some(info.deep_progress),
        }),
        time_this_week: Some(info.time_this_week),
    })
}

pub async fn weekwalk_v2_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<WeekwalkVer2GetInfoReply, AppError> {
    Ok(WeekwalkVer2GetInfoReply {
        info: Some(weekwalk_v2::get_weekwalk_v2_info(db, player_id).await?),
    })
}
