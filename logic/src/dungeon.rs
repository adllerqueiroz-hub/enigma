use crate::reward::{self, RewardSet};

pub struct CompletionRewards {
    pub rewards: RewardSet,
    pub player_exp: i32,
    pub first_bonus: Vec<(u32, u32, i32)>,
    pub normal_bonus: Vec<(u32, u32, i32)>,
    pub advanced_bonus: Vec<(u32, u32, i32)>,
}

pub fn completion_rewards(
    episode: &config::episode::Episode,
    first_pass: bool,
    previous_star: i32,
    star: i32,
    multiplier: i32,
) -> CompletionRewards {
    let cost = episode_cost_value(episode);
    let player_exp = episode_player_exp(episode, first_pass, multiplier);
    let mut normal_rewards = reward::parse_bonus_with_cost(episode.bonus, cost);
    normal_rewards.scale(multiplier);
    let first_rewards = if first_pass {
        reward::parse_bonus_with_cost(episode.first_bonus, cost)
    } else {
        Default::default()
    };
    let advanced_rewards = if previous_star < 2 && star >= 2 {
        reward::parse_bonus_with_cost(episode.advanced_bonus, cost)
    } else {
        Default::default()
    };
    let normal_bonus = normal_rewards.material_changes();
    let first_bonus = first_rewards.material_changes();
    let advanced_bonus = advanced_rewards.material_changes();
    let mut rewards = normal_rewards;
    rewards.extend(first_rewards);
    rewards.extend(advanced_rewards);
    rewards.player_exp = rewards.player_exp.saturating_add(player_exp);

    CompletionRewards {
        rewards,
        player_exp,
        first_bonus,
        normal_bonus,
        advanced_bonus,
    }
}

pub fn episode_player_exp(
    episode: &config::episode::Episode,
    first_pass: bool,
    multiplier: i32,
) -> i32 {
    let cost = episode_cost_value(episode);
    let normal = config::configs::get()
        .bonus
        .get(episode.bonus)
        .map(|bonus| player_exp_value(&bonus.player_exp, cost))
        .unwrap_or_default()
        .saturating_mul(multiplier);
    let first = first_pass
        .then(|| config::configs::get().bonus.get(episode.first_bonus))
        .flatten()
        .map(|bonus| player_exp_value(&bonus.player_exp, cost))
        .unwrap_or_default();

    normal.saturating_add(first)
}

pub fn episode_cost_value(episode: &config::episode::Episode) -> i32 {
    episode
        .cost
        .split('|')
        .find_map(|part| part.rsplit('#').next()?.parse::<i32>().ok())
        .unwrap_or_default()
}

fn player_exp_value(value: &str, cost: i32) -> i32 {
    value.parse().unwrap_or_else(|_| {
        value
            .strip_suffix("*cost")
            .and_then(|factor| factor.parse::<i32>().ok())
            .map(|factor| factor.saturating_mul(cost))
            .unwrap_or_default()
    })
}
