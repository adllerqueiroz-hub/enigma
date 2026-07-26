use super::*;
use database::models::game::heros::UserHeroModel;
use sonettobuf::Act198GainReply;

const STATE_ENTRY_ID: i32 = 0;

pub struct Act198Claim {
    pub reply: Act198GainReply,
    pub rewards: Option<reward::AppliedRewards>,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn act198_can_gain(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Option<i32>, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let row = activity_config(activity_id)?;
    if already_claimed(db, player_id, activity_id).await? {
        return Ok(None);
    }

    let owned_skins = UserHeroModel::new(player_id, db.clone())
        .get_skins()
        .await?;
    let owned_count = row
        .skin_ids
        .split('#')
        .filter_map(|id| id.parse::<i32>().ok())
        .filter(|id| owned_skins.contains(id))
        .count() as i32;

    Ok((owned_count >= row.num).then_some(activity_id))
}

pub async fn act198_gain(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Act198Claim, AppError> {
    let activity_id = act198_can_gain(db, player_id, activity_id)
        .await?
        .ok_or(AppError::InvalidRequest)?;
    let row = activity_config(activity_id)?;
    let parsed = reward::parse(&row.bonus);
    let material_changes = parsed.material_changes();
    let rewards = reward::apply(db, player_id, parsed).await?;

    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act198Gain,
            entry_id: STATE_ENTRY_ID,
            state: 1,
            progress: 0,
            ext: "",
        },
    )
    .await?;

    Ok(Act198Claim {
        reply: Act198GainReply {
            activity_id: Some(activity_id),
        },
        rewards: Some(rewards),
        material_changes,
    })
}

fn resolve_activity_id(activity_id: Option<i32>) -> Result<i32, AppError> {
    activity_id
        .or_else(|| {
            config::configs::get()
                .activity198
                .iter()
                .map(|row| row.activity_id)
                .max()
        })
        .ok_or(AppError::InvalidRequest)
}

fn activity_config(
    activity_id: i32,
) -> Result<&'static config::activity198::Activity198, AppError> {
    config::configs::get()
        .activity198
        .iter()
        .find(|row| row.activity_id == activity_id)
        .ok_or(AppError::InvalidRequest)
}

async fn already_claimed(
    db: &SqlitePool,
    player_id: i64,
    activity_id: i32,
) -> Result<bool, AppError> {
    Ok(
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act198Gain)
            .await?
            .get(&STATE_ENTRY_ID)
            .is_some_and(|(state, _, _)| *state != 0),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn generated_activity198_config_loads_skin_gate() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);
        let row = config::configs::get()
            .activity198
            .iter()
            .find(|row| row.activity_id == 12882)
            .expect("activity198 exists");

        assert_eq!(row.num, 1);
        assert!(row.skin_ids.split('#').any(|id| id == "308603"));
        assert!(
            !crate::reward::parse(&row.bonus)
                .material_changes()
                .is_empty()
        );
    }
}
