use super::*;

pub struct Act189OnceBonusClaim {
    pub reply: GetAct189OnceBonusReply,
    pub rewards: Option<reward::AppliedRewards>,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn act189_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<GetAct189InfoReply, AppError> {
    let activity_id = activity_id.unwrap_or_else(latest_act189_activity_id);
    let states = activity_state::get(
        db,
        player_id,
        activity_id,
        ActivityStateKind::Act189OnceBonus,
    )
    .await?;

    Ok(GetAct189InfoReply {
        activity_id: Some(activity_id),
        has_get_once_bonus: Some(states.get(&0).map(|(state, _, _)| *state).unwrap_or(0) != 0),
    })
}

pub async fn get_act189_once_bonus(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Act189OnceBonusClaim, AppError> {
    let activity_id = activity_id.unwrap_or_else(latest_act189_activity_id);
    let row = config::configs::get()
        .activity189
        .iter()
        .find(|row| row.activity_id == activity_id)
        .ok_or(AppError::InvalidRequest)?;
    let states = activity_state::get(
        db,
        player_id,
        activity_id,
        ActivityStateKind::Act189OnceBonus,
    )
    .await?;

    let (rewards, material_changes) =
        if states.get(&0).map(|(state, _, _)| *state).unwrap_or(0) == 0 {
            activity_state::set(
                db,
                player_id,
                activity_id,
                ActivityStateSet {
                    kind: ActivityStateKind::Act189OnceBonus,
                    entry_id: 0,
                    state: 1,
                    progress: 0,
                    ext: "",
                },
            )
            .await?;

            let parsed = reward::parse(&row.bonus);
            let material_changes = parsed.material_changes();
            let rewards = reward::apply(db, player_id, parsed).await?;

            (Some(rewards), material_changes)
        } else {
            (None, Vec::new())
        };

    Ok(Act189OnceBonusClaim {
        reply: GetAct189OnceBonusReply {
            activity_id: Some(activity_id),
        },
        rewards,
        material_changes,
    })
}

fn latest_act189_activity_id() -> i32 {
    config::configs::get()
        .activity189
        .iter()
        .map(|row| row.activity_id)
        .max()
        .unwrap_or_default()
}
