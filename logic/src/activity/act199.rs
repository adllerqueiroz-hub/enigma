use super::*;
use database::db::game::activity199;
use sonettobuf::{Act199GainReply, Get199InfoReply};

pub struct Act199GainClaim {
    pub reply: Act199GainReply,
    pub rewards: Option<reward::AppliedRewards>,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn act199_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Get199InfoReply, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    activity199::sync(db, player_id, activity_id).await?;
    let hero_id = activity199::get(db, player_id, activity_id).await?;

    Ok(Get199InfoReply {
        activity_id: Some(activity_id),
        hero_id: Some(hero_id),
    })
}

pub async fn act199_gain(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    hero_id: Option<i32>,
) -> Result<Act199GainClaim, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let hero_id = hero_id.ok_or(AppError::InvalidRequest)?;
    if !is_allowed_hero(activity_id, hero_id) {
        return Err(AppError::InvalidRequest);
    }

    let selected_hero_id = activity199::get(db, player_id, activity_id).await?;
    if selected_hero_id != 0 {
        return Ok(Act199GainClaim {
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
    activity199::set_hero(db, player_id, activity_id, hero_id).await?;

    Ok(Act199GainClaim {
        reply: reply(activity_id, hero_id),
        rewards: Some(rewards),
        material_changes,
    })
}

fn resolve_activity_id(activity_id: Option<i32>) -> Result<i32, AppError> {
    activity_id
        .or_else(|| {
            config::configs::get()
                .activity199
                .iter()
                .map(|row| row.activity_id)
                .max()
        })
        .ok_or(AppError::InvalidRequest)
}

fn is_allowed_hero(activity_id: i32, hero_id: i32) -> bool {
    config::configs::get()
        .activity199
        .iter()
        .find(|row| row.activity_id == activity_id)
        .is_some_and(|row| {
            row.hero_ids
                .split('#')
                .filter_map(|id| id.parse::<i32>().ok())
                .any(|id| id == hero_id)
        })
}

fn reply(activity_id: i32, hero_id: i32) -> Act199GainReply {
    Act199GainReply {
        activity_id: Some(activity_id),
        hero_id: Some(hero_id),
    }
}
