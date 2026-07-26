use super::*;
use database::db::game::activity225;
use sonettobuf::GetAct225InfoReply;

pub async fn act225_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<GetAct225InfoReply, AppError> {
    let activity_id = activity_id.unwrap_or_else(default_activity_id);
    let state = activity225::get(db, player_id, activity_id).await?;

    Ok(GetAct225InfoReply {
        activity_id: Some(activity_id),
        last_red_envelope_rain_id: Some(state.last_red_envelope_rain_id),
        question_id: Some(state.question_id),
        rock_paper_scissors_daily_count: Some(state.rock_paper_scissors_daily_count),
    })
}

fn default_activity_id() -> i32 {
    config::configs::get()
        .activity225_const
        .iter()
        .map(|row| row.activity_id)
        .max()
        .unwrap_or_default()
}
