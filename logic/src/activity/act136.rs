use super::*;
use sonettobuf::{Act136SelectReply, Get136InfoReply};

pub struct Act136SelectClaim {
    pub reply: Act136SelectReply,
    pub rewards: Option<reward::AppliedRewards>,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn act136_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Get136InfoReply, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let selected_hero_id = selected_hero_id(db, player_id, activity_id).await?;

    Ok(Get136InfoReply {
        activity_id: Some(activity_id),
        select_hero_id: Some(selected_hero_id),
    })
}

pub async fn act136_select(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    hero_id: Option<i32>,
) -> Result<Act136SelectClaim, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let hero_id = hero_id.ok_or(AppError::InvalidRequest)?;
    if !is_allowed_hero(activity_id, hero_id) {
        return Err(AppError::InvalidRequest);
    }

    let selected_hero_id = selected_hero_id(db, player_id, activity_id).await?;
    if selected_hero_id != 0 {
        return Ok(Act136SelectClaim {
            reply: reply(activity_id, selected_hero_id),
            rewards: None,
            material_changes: Vec::new(),
        });
    }

    let parsed = reward::RewardSet {
        heroes: vec![(hero_id, 1)],
        ..Default::default()
    };
    let material_changes = parsed.material_changes();
    let rewards = reward::apply(db, player_id, parsed).await?;
    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act136Select,
            entry_id: 0,
            state: hero_id,
            progress: 0,
            ext: "",
        },
    )
    .await?;

    Ok(Act136SelectClaim {
        reply: reply(activity_id, hero_id),
        rewards: Some(rewards),
        material_changes,
    })
}

fn resolve_activity_id(activity_id: Option<i32>) -> Result<i32, AppError> {
    activity_id
        .or_else(|| {
            config::configs::get()
                .activity136
                .iter()
                .map(|row| row.activity_id)
                .max()
        })
        .ok_or(AppError::InvalidRequest)
}

async fn selected_hero_id(
    db: &SqlitePool,
    player_id: i64,
    activity_id: i32,
) -> Result<i32, AppError> {
    Ok(
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act136Select)
            .await?
            .get(&0)
            .map(|(state, _, _)| *state)
            .unwrap_or(0),
    )
}

fn is_allowed_hero(activity_id: i32, hero_id: i32) -> bool {
    config::configs::get()
        .activity136
        .iter()
        .find(|row| row.activity_id == activity_id)
        .is_some_and(|row| {
            row.hero_ids
                .split('#')
                .filter_map(|id| id.parse::<i32>().ok())
                .any(|id| id == hero_id)
        })
}

fn reply(activity_id: i32, hero_id: i32) -> Act136SelectReply {
    Act136SelectReply {
        activity_id: Some(activity_id),
        select_hero_id: Some(hero_id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_hero_uses_activity136_config() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);

        assert!(is_allowed_hero(13202, 3118));
        assert!(!is_allowed_hero(13202, 9999));
    }
}
