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
            let mut card = card_for(entity, entity.skill_group1.first().copied())?;
            card.target_uid = target_uids
                .get(rng.random_range(0..target_uids.len()))
                .copied();
            Some(card)
        })
        .collect()
}
