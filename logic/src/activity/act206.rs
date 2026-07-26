use super::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sonettobuf::{
    Act206ChooseDirectionReply, Act206ChosenInfo, Act206GetBonusReply, Act206GetInfoReply,
};

const STATE_ENTRY_ID: i32 = 0;

#[derive(Clone, Copy, Deserialize, Serialize)]
struct ChosenState {
    direction_id: i32,
    direction_gen_time: u64,
    reselected_num: i32,
    reward_id: i32,
}

pub struct Act206Claim {
    pub reply: Act206GetBonusReply,
    pub rewards: reward::AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn act206_get_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Act206GetInfoReply, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let chosen = current_choice(db, player_id, activity_id).await?;

    Ok(Act206GetInfoReply {
        activity_id: Some(activity_id),
        has_chosen_direction: Some(chosen.is_some()),
        chosen_info: chosen.map(chosen_info),
        option_directions: option_directions(activity_id),
    })
}

pub async fn act206_choose_direction(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    direction_id: Option<i32>,
) -> Result<Act206ChooseDirectionReply, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let direction_id = direction_id.ok_or(AppError::InvalidRequest)?;
    let direction = config::configs::get()
        .activity206_reward_direction
        .iter()
        .find(|row| row.activity_id == activity_id && row.direction_id == direction_id)
        .ok_or(AppError::InvalidRequest)?;
    let group = config::configs::get()
        .activity206_reward_group
        .by_group(direction.reward_group_id)
        .next()
        .ok_or(AppError::InvalidRequest)?;
    let reward_id =
        choose_weighted_reward(&group.reward_id2_prob).ok_or(AppError::InvalidRequest)?;
    let chosen = ChosenState {
        direction_id,
        direction_gen_time: common::time::ServerTime::now_ms() as u64,
        reselected_num: 0,
        reward_id,
    };
    let ext = serde_json::to_string(&chosen)?;

    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act206Chosen,
            entry_id: STATE_ENTRY_ID,
            state: 1,
            progress: 0,
            ext: &ext,
        },
    )
    .await?;

    Ok(Act206ChooseDirectionReply {
        activity_id: Some(activity_id),
        chosen_info: Some(chosen_info(chosen)),
    })
}

pub async fn act206_get_bonus(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Act206Claim, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let chosen = current_choice(db, player_id, activity_id)
        .await?
        .ok_or(AppError::InvalidRequest)?;
    let row = config::configs::get()
        .activity206_reward
        .iter()
        .find(|row| row.reward_id == chosen.reward_id)
        .ok_or(AppError::InvalidRequest)?;

    let parsed = reward::parse(&row.reward);
    let material_changes = parsed.material_changes();
    let rewards = reward::apply(db, player_id, parsed).await?;
    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act206Chosen,
            entry_id: STATE_ENTRY_ID,
            state: 2,
            progress: 0,
            ext: "",
        },
    )
    .await?;

    Ok(Act206Claim {
        reply: Act206GetBonusReply {
            activity_id: Some(activity_id),
            reward_id: Some(chosen.reward_id),
        },
        rewards,
        material_changes,
    })
}

fn resolve_activity_id(activity_id: Option<i32>) -> Result<i32, AppError> {
    activity_id
        .or_else(|| {
            config::configs::get()
                .activity206_reward_direction
                .iter()
                .map(|row| row.activity_id)
                .max()
        })
        .ok_or(AppError::InvalidRequest)
}

async fn current_choice(
    db: &SqlitePool,
    player_id: i64,
    activity_id: i32,
) -> Result<Option<ChosenState>, AppError> {
    Ok(
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act206Chosen)
            .await?
            .get(&STATE_ENTRY_ID)
            .filter(|(state, _, _)| *state == 1)
            .and_then(|(_, _, ext)| serde_json::from_str(ext).ok()),
    )
}

fn option_directions(activity_id: i32) -> Vec<i32> {
    let mut directions = config::configs::get()
        .activity206_reward_direction
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .map(|row| row.direction_id)
        .collect::<Vec<_>>();
    directions.sort_unstable();
    directions
}

fn chosen_info(chosen: ChosenState) -> Act206ChosenInfo {
    Act206ChosenInfo {
        current_direction: Some(chosen.direction_id),
        direction_gen_time: Some(chosen.direction_gen_time),
        reselected_num: Some(chosen.reselected_num),
        reward_id: Some(chosen.reward_id),
    }
}

fn choose_weighted_reward(value: &str) -> Option<i32> {
    let rewards = parse_weighted(value);
    let total = rewards.iter().map(|(_, weight)| *weight).sum::<u32>();
    if total == 0 {
        return None;
    }

    let mut roll = rand::rng().random_range(0..total);
    for (id, weight) in rewards {
        if roll < weight {
            return Some(id);
        }
        roll -= weight;
    }

    None
}

fn parse_weighted(value: &str) -> Vec<(i32, u32)> {
    value
        .split('|')
        .filter_map(|part| {
            let mut fields = part.split('#');
            Some((fields.next()?.parse().ok()?, fields.next()?.parse().ok()?))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn act206_weighted_rewards_parse() {
        assert_eq!(
            parse_weighted("1#30|2#40|7#30"),
            vec![(1, 30), (2, 40), (7, 30)]
        );
        assert_eq!(choose_weighted_reward("3#100"), Some(3));
    }
}
