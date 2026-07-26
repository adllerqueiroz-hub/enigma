use super::*;

pub struct Act208Claim {
    pub reply: Act208ReceiveBonusReply,
    pub rewards: Option<reward::AppliedRewards>,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn act208_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<GetAct208InfoReply, AppError> {
    let activity_id = activity_id.unwrap_or_else(|| {
        config::configs::get()
            .activity208_bonus
            .iter()
            .map(|row| row.activity_id)
            .max()
            .unwrap_or_default()
    });
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act208Bonus).await?;

    Ok(GetAct208InfoReply {
        activity_id: Some(activity_id),
        bonus: config::configs::get()
            .activity208_bonus
            .iter()
            .filter(|row| row.activity_id == activity_id)
            .map(|row| Act208BonusNo {
                id: Some(row.id),
                status: Some(states.get(&row.id).map(|(state, _, _)| *state).unwrap_or(0)),
            })
            .collect(),
    })
}

pub async fn receive_act208_bonus(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    id: Option<i32>,
) -> Result<Act208Claim, AppError> {
    let activity_id = activity_id.ok_or(AppError::InvalidRequest)?;
    let id = id.ok_or(AppError::InvalidRequest)?;
    let row = config::configs::get()
        .activity208_bonus
        .get(id)
        .filter(|row| row.activity_id == activity_id)
        .ok_or(AppError::InvalidRequest)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act208Bonus).await?;
    if states.get(&id).map(|(state, _, _)| *state).unwrap_or(0) != 1 {
        return Err(AppError::InvalidRequest);
    }

    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act208Bonus,
            entry_id: id,
            state: 2,
            progress: 0,
            ext: "",
        },
    )
    .await?;
    let parsed = reward::parse(&row.bonus);
    let material_changes = parsed.material_changes();
    let rewards = Some(reward::apply(db, player_id, parsed).await?);

    Ok(Act208Claim {
        reply: Act208ReceiveBonusReply {
            activity_id: Some(activity_id),
            id: Some(id),
        },
        rewards,
        material_changes,
    })
}
