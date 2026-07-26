use rand::Rng;
use sonettobuf::{CardInfo, Fight};

use super::pool::{active_enemy_entities, active_player_uids, card_for};

pub fn generate_ai_deck<R: Rng + ?Sized>(fight: &Fight, rng: &mut R) -> Vec<CardInfo> {
    let enemies = active_enemy_entities(fight);
    if enemies.is_empty() {
        return Vec::new();
    }

    let target_uids = active_player_uids(fight);
    if target_uids.is_empty() {
        return Vec::new();
    }
    enemies
        .into_iter()
        .filter_map(|entity| {
            let mut card = card_for(entity, select_skill(entity))?;
            card.target_uid = target_uids
                .get(rng.random_range(0..target_uids.len()))
                .copied();
            Some(card)
        })
        .collect()
}

fn select_skill(entity: &sonettobuf::FightEntityInfo) -> Option<i32> {
    let ultimate = entity.ex_skill.filter(|skill_id| *skill_id > 0);
    let required = ultimate
        .map(crate::engine::skill::effect::catalog::configured_big_skill_point)
        .filter(|cost| *cost > 0)
        .unwrap_or(5)
        .saturating_add(entity.expoint_max_add.unwrap_or_default());
    if let Some(ultimate) = ultimate
        && entity.ex_point.unwrap_or_default() >= required
    {
        return Some(ultimate);
    }

    entity
        .skill_group1
        .first()
        .or(entity.skill_group2.first())
        .copied()
}
