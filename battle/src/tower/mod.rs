use database::models::game::tower::{TowerConstId, TowerType};
use sonettobuf::{StartDungeonRequest, StartTowerBattleRequest};

mod fight;

pub use fight::{build_fight, system_plan_rule_skills};

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BattleContext {
    pub tower_type: i32,
    pub tower_id: i32,
    pub layer_id: i32,
    pub difficulty: i32,
    pub talent_plan_id: i32,
}

pub fn validate_battle_start(
    tables: &config::GameDB,
    request: &StartTowerBattleRequest,
) -> anyhow::Result<(StartDungeonRequest, BattleContext)> {
    let dungeon = request
        .start_dungeon_request
        .clone()
        .ok_or_else(|| anyhow::anyhow!("tower battle has no dungeon request"))?;
    let episode_id = dungeon
        .episode_id
        .ok_or_else(|| anyhow::anyhow!("tower battle has no episode"))?;
    let context = BattleContext {
        tower_type: request
            .r#type
            .ok_or_else(|| anyhow::anyhow!("tower battle has no tower type"))?,
        tower_id: request.tower_id.unwrap_or_default(),
        layer_id: request.layer_id.unwrap_or_default(),
        difficulty: request.difficulty.unwrap_or_default(),
        talent_plan_id: request.talent_plan_id.unwrap_or_default(),
    };
    let boss_id = dungeon
        .fight_group
        .as_ref()
        .and_then(|group| group.assist_boss_id)
        .unwrap_or_default();
    let custom_plan_count = tables
        .tower_const
        .get(TowerConstId::CustomTalentPlanCount.id())
        .and_then(|row| row.value.parse().ok())
        .unwrap_or_default();
    let valid_boss = boss_id == 0
        || (tables
            .tower_assist_boss
            .iter()
            .any(|row| row.boss_id == boss_id)
            && ((1..=custom_plan_count).contains(&context.talent_plan_id)
                || tables
                    .tower_talent_plan
                    .iter()
                    .any(|row| row.boss_id == boss_id && row.plan_id == context.talent_plan_id)));

    let valid_episode = match context.tower_type {
        value if value == TowerType::Normal.id() => {
            context.tower_id == 0
                && tables.tower_permanent_episode.iter().any(|row| {
                    row.layer_id == context.layer_id
                        && row
                            .episode_ids
                            .split('|')
                            .any(|id| id.parse::<i32>() == Ok(episode_id))
                })
        }
        value if value == TowerType::Boss.id() => {
            tables.tower_boss_episode.iter().any(|row| {
                row.tower_id == context.tower_id
                    && row.layer_id == context.layer_id
                    && row.episode_id == episode_id
            }) || (context.layer_id == 0
                && tables.tower_boss_teach.iter().any(|row| {
                    row.tower_id == context.tower_id
                        && row.teach_id == context.difficulty
                        && row.episode_id == episode_id
                }))
        }
        value if value == TowerType::Limited.id() => {
            tables.tower_limited_episode.iter().any(|row| {
                row.season == context.tower_id
                    && row.layer_id == context.layer_id
                    && row.difficulty == context.difficulty
                    && row.episode_id == episode_id
            })
        }
        _ => false,
    };

    anyhow::ensure!(
        valid_episode && valid_boss,
        "invalid tower battle selection"
    );
    Ok((dungeon, context))
}

#[cfg(test)]
mod test;
