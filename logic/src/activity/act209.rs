use super::*;

pub async fn act209_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<GetAct209InfoReply, AppError> {
    let activity_id = activity_id.unwrap_or_else(|| {
        config::configs::get()
            .activity209_task
            .iter()
            .map(|row| row.activity_id)
            .max()
            .unwrap_or_default()
    });
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act209Layer).await?;

    Ok(GetAct209InfoReply {
        activity_id: Some(activity_id),
        max_layer: Some(
            states
                .get(&0)
                .map(|(_, progress, _)| *progress)
                .unwrap_or(0),
        ),
    })
}
