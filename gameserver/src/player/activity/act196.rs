use super::*;
use sonettobuf::{Act196GainReply, Get196InfoReply};

pub struct Act196Claim {
    pub reply: Act196GainReply,
    pub rewards: Option<reward::AppliedRewards>,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn act196_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Get196InfoReply, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act196Gain).await?;

    Ok(Get196InfoReply {
        activity_id: Some(activity_id),
        has_gain: states
            .iter()
            .filter_map(|(id, (state, _, _))| (*state != 0).then_some(*id))
            .collect(),
    })
}

pub async fn act196_gain(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    id: Option<i32>,
) -> Result<Act196Claim, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let id = id.ok_or(AppError::InvalidRequest)?;
    let row = config::configs::get()
        .activity196
        .iter()
        .find(|row| row.activity_id == activity_id && row.id == id)
        .ok_or(AppError::InvalidRequest)?;

    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act196Gain).await?;
    if states.get(&id).is_some_and(|(state, _, _)| *state != 0) {
        return Ok(Act196Claim {
            reply: reply(activity_id, id),
            rewards: None,
            material_changes: Vec::new(),
        });
    }

    let parsed = reward::parse(&row.bonus);
    let material_changes = parsed.material_changes();
    let rewards = reward::apply(db, player_id, parsed).await?;
    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act196Gain,
            entry_id: id,
            state: 1,
            progress: 0,
            ext: "",
        },
    )
    .await?;

    Ok(Act196Claim {
        reply: reply(activity_id, id),
        rewards: Some(rewards),
        material_changes,
    })
}

fn resolve_activity_id(activity_id: Option<i32>) -> Result<i32, AppError> {
    activity_id
        .or_else(|| {
            config::configs::get()
                .activity196
                .iter()
                .map(|row| row.activity_id)
                .max()
        })
        .ok_or(AppError::InvalidRequest)
}

fn reply(activity_id: i32, id: i32) -> Act196GainReply {
    Act196GainReply {
        activity_id: Some(activity_id),
        id: Some(id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activity196_rewards_are_config_driven() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);
        let row = config::configs::get()
            .activity196
            .iter()
            .find(|row| row.activity_id == 12862 && row.id == 1)
            .expect("activity196 reward exists");

        assert!(!reward::parse(&row.bonus).material_changes().is_empty());
    }
}
