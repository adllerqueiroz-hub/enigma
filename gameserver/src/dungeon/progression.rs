use super::*;

pub struct InstructionDungeonRewardClaim {
    pub reply: InstructionDungeonRewardReply,
    pub rewards: AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub struct InstructionDungeonFinalRewardClaim {
    pub reply: InstructionDungeonFinalRewardReply,
    pub rewards: AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn can_start_episode(
    db: &SqlitePool,
    player_id: i64,
    chapter_id: i32,
    episode_id: i32,
) -> Result<bool, AppError> {
    Ok(dungeons::can_start_episode(db, player_id, chapter_id, episode_id).await?)
}

pub fn episode_player_exp(
    episode: &config::episode::Episode,
    first_pass: bool,
    multiplier: i32,
) -> i32 {
    let cost = episode_cost_value(episode);

    let normal = configs::get()
        .bonus
        .get(episode.bonus)
        .map(|bonus| player_exp_value(&bonus.player_exp, cost))
        .unwrap_or_default()
        .saturating_mul(multiplier);
    let first = first_pass
        .then(|| configs::get().bonus.get(episode.first_bonus))
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

pub fn episode_cost(episode: &config::episode::Episode, multiplier: i32) -> reward::RewardSet {
    let mut cost = reward::parse(&episode.cost);
    cost.scale(multiplier.max(1));
    cost
}

pub fn failure_refund(episode: &config::episode::Episode, multiplier: i32) -> reward::RewardSet {
    let mut refund = episode_cost(episode, multiplier);
    let retained = reward::parse(&episode.fail_cost);
    subtract_costs(&mut refund.items, &retained.items);
    subtract_costs(&mut refund.currencies, &retained.currencies);
    refund
}

fn subtract_costs<T: Eq>(costs: &mut Vec<(T, i32)>, retained: &[(T, i32)]) {
    for (id, amount) in retained {
        if let Some((_, refundable)) = costs.iter_mut().find(|(cost_id, _)| cost_id == id) {
            *refundable = (*refundable - amount).max(0);
        }
    }
    costs.retain(|(_, amount)| *amount > 0);
}

#[derive(Clone, Copy)]
enum AdvancedConditionType {
    CasualtiesBelow = 1,
    RoundsAtMost = 2,
    NoCasualtiesWithinRounds = 3,
}

impl AdvancedConditionType {
    fn from_id(id: i32) -> Option<Self> {
        match id {
            1 => Some(Self::CasualtiesBelow),
            2 => Some(Self::RoundsAtMost),
            3 => Some(Self::NoCasualtiesWithinRounds),
            _ => None,
        }
    }
}

pub fn battle_star(runtime: &battle::engine::runtime::BattleRuntime, battle_id: i32) -> i32 {
    let Some(battle) = configs::get().battle.get(battle_id) else {
        return 1;
    };
    let dead = runtime.dead_attacker_count() as i32;
    let round = runtime.current_round();

    1 + battle
        .advanced_condition
        .split('|')
        .filter_map(|id| id.parse::<i32>().ok())
        .filter_map(|id| configs::get().condition.get(id))
        .filter(|condition| {
            let limit = condition.attr.parse::<i32>().unwrap_or_default();
            match AdvancedConditionType::from_id(condition.r#type) {
                Some(AdvancedConditionType::CasualtiesBelow) => dead < limit,
                Some(AdvancedConditionType::RoundsAtMost) => round <= limit,
                Some(AdvancedConditionType::NoCasualtiesWithinRounds) => {
                    dead == 0 && round <= limit
                }
                None => {
                    tracing::warn!(
                        condition_id = condition.id,
                        condition_type = condition.r#type,
                        "unsupported dungeon advanced condition"
                    );
                    false
                }
            }
        })
        .count() as i32
}
