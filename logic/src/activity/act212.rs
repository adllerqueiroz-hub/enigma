use super::*;

pub struct Act212Claim {
    pub reply: Act212ReceiveBonusReply,
    pub rewards: reward::AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn act212_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<GetAct212InfoReply, AppError> {
    let act212_info = act212_info_no(db, player_id, activity_id).await?;
    Ok(GetAct212InfoReply {
        act212_info: Some(act212_info),
    })
}

pub async fn receive_act212_bonus(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    id: Option<i32>,
) -> Result<Act212Claim, AppError> {
    let activity_id = activity_id.unwrap_or_else(latest_act212_activity_id);
    let id = id.ok_or(AppError::InvalidRequest)?;
    let row = config::configs::get()
        .activity212_bonus
        .iter()
        .find(|row| row.activity_id == activity_id && row.id == id)
        .ok_or(AppError::InvalidRequest)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act212Bonus).await?;
    if states.get(&id).map(|(state, _, _)| *state).unwrap_or(0) != 1 {
        return Err(AppError::InvalidRequest);
    }

    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act212Bonus,
            entry_id: id,
            state: 2,
            progress: 0,
            ext: "",
        },
    )
    .await?;

    let parsed = reward::parse(&row.bonus);
    let material_changes = parsed.material_changes();
    let rewards = reward::apply(db, player_id, parsed).await?;

    Ok(Act212Claim {
        reply: Act212ReceiveBonusReply {
            activity_id: Some(activity_id),
            id: Some(id),
            status: Some(2),
        },
        rewards,
        material_changes,
    })
}

async fn act212_info_no(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Act212InfoNo, AppError> {
    let activity_id = activity_id.unwrap_or_else(latest_act212_activity_id);
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act212Bonus).await?;
    let mut bonuss = config::configs::get()
        .activity212_bonus
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .map(|row| Act212BonusNo {
            id: Some(row.id),
            status: Some(
                states
                    .get(&row.id)
                    .map(|(state, _, _)| *state)
                    .unwrap_or_else(|| i32::from(row.id == 1)),
            ),
        })
        .collect::<Vec<_>>();
    bonuss.sort_by_key(|bonus| bonus.id.unwrap_or_default());

    let end_time = config::configs::get()
        .activity212_const
        .iter()
        .find(|row| row.activity_id == activity_id)
        .map(|row| row.value as i64 * 86_400_000)
        .unwrap_or(0);

    Ok(Act212InfoNo {
        activity_id: Some(activity_id),
        is_active: Some(!bonuss.is_empty()),
        bonuss,
        end_time: Some(end_time),
    })
}
