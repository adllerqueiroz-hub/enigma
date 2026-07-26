use rand::{SeedableRng, rngs::StdRng};
use sonettobuf::{CardInfo, Fight};

use crate::engine::manager::card::{
    ai::generate_ai_deck,
    draw::draw_guaranteed_by_uid,
    pool::{active_enemy_entities, active_player_uids, card_for, player_candidate_pool},
};

const CARDS_PER_HERO: i32 = 16;
const MAX_NORMAL_HAND_SIZE: usize = 8;

pub fn deck_size(fight: &Fight) -> i32 {
    let normal_uids =
        crate::engine::manager::card::pool::normal_player_candidate_pool_with(fight, |_| false)
            .into_iter()
            .filter_map(|card| card.uid)
            .collect::<std::collections::HashSet<_>>();
    i32::try_from(normal_uids.len())
        .unwrap_or(i32::MAX)
        .saturating_mul(CARDS_PER_HERO)
}

pub fn hand_size(fight: &Fight) -> usize {
    hand_size_from_count(active_player_uids(fight).len())
}

pub fn hand_size_from_count(characters: usize) -> usize {
    match characters {
        0 => 0,
        characters => (characters * 2 + 1).min(MAX_NORMAL_HAND_SIZE),
    }
}

pub fn draw_bag(fight: &Fight) -> Vec<CardInfo> {
    let candidates =
        crate::engine::manager::card::pool::normal_player_candidate_pool_with(fight, |_| false);
    let mut cards = active_player_uids(fight)
        .into_iter()
        .flat_map(|uid| {
            let owner = candidates
                .iter()
                .filter(|card| card.uid == Some(uid))
                .cloned()
                .collect::<Vec<_>>();
            (0..CARDS_PER_HERO)
                .filter_map(move |index| owner.get(index as usize % owner.len().max(1)).cloned())
        })
        .collect::<Vec<_>>();
    cards.extend(crate::engine::manager::card::pool::device_draw_bag(fight));
    cards
}

pub fn start_decks_from_fight(
    fight: &Fight,
    seed_value: i32,
    captured: Option<(Vec<CardInfo>, Vec<CardInfo>)>,
) -> (Vec<CardInfo>, Vec<CardInfo>) {
    let required_uids = active_player_uids(fight);
    let valid_target_uids = fight
        .attacker
        .iter()
        .chain(&fight.defender)
        .flat_map(|team| &team.entitys)
        .filter(|entity| entity.current_hp.unwrap_or(1) > 0)
        .filter_map(|entity| entity.uid)
        .collect::<std::collections::HashSet<_>>();
    let candidates = player_candidate_pool(fight);
    let hand_size = hand_size(fight);
    let mut rng = StdRng::seed_from_u64(seed(fight, seed_value));
    if let Some((captured_ai, captured_player)) = captured {
        let ai_candidates = active_enemy_entities(fight)
            .into_iter()
            .flat_map(|entity| {
                entity
                    .skill_group1
                    .iter()
                    .chain(&entity.skill_group2)
                    .copied()
                    .chain(entity.ex_skill)
                    .filter_map(|skill_id| card_for(entity, Some(skill_id)))
            })
            .collect::<Vec<_>>();
        let ai = captured_ai
            .iter()
            .filter_map(|captured| {
                let mut candidate = ai_candidates
                    .iter()
                    .find(|candidate| {
                        captured.uid == candidate.uid && captured.skill_id == candidate.skill_id
                    })?
                    .clone();
                candidate.target_uid = captured
                    .target_uid
                    .filter(|uid| valid_target_uids.contains(uid))
                    .or(candidate.target_uid);
                Some(candidate)
            })
            .collect();
        let player = captured_player
            .iter()
            .filter_map(|captured| {
                candidates
                    .iter()
                    .find(|candidate| {
                        captured.uid == candidate.uid && captured.skill_id == candidate.skill_id
                    })
                    .cloned()
            })
            .collect();
        return (ai, player);
    }

    let player = draw_guaranteed_by_uid(&candidates, &required_uids, hand_size, &mut rng);
    let ai = generate_ai_deck(fight, &mut rng);
    (ai, player)
}

fn seed(fight: &Fight, seed_value: i32) -> u64 {
    let mut seed = 1_469_598_103_934_665_603_u64 ^ seed_value as u64;
    for uid in active_player_uids(fight) {
        seed = (seed ^ uid as u64).wrapping_mul(1_099_511_628_211);
        if let Some(entity) = fight
            .attacker
            .as_ref()
            .and_then(|team| team.entitys.iter().find(|entity| entity.uid == Some(uid)))
        {
            seed =
                (seed ^ entity.model_id.unwrap_or_default() as u64).wrapping_mul(1_099_511_628_211);
            seed = (seed ^ entity.skill_group1.first().copied().unwrap_or_default() as u64)
                .wrapping_mul(1_099_511_628_211);
            seed = (seed ^ entity.skill_group2.first().copied().unwrap_or_default() as u64)
                .wrapping_mul(1_099_511_628_211);
        }
    }
    seed
}

#[cfg(test)]
mod tests {
    use sonettobuf::{Fight, FightEntityInfo, FightTeam};

    use super::*;

    #[test]
    fn builds_start_decks_from_fight_entities() {
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![
                    entity(10, 1001, 2, &[101], &[201]),
                    entity(11, 1002, 1, &[102], &[202]),
                ],
                ..Default::default()
            }),
            defender: Some(FightTeam {
                entitys: vec![entity(-1, 2001, 1, &[301], &[401])],
                ..Default::default()
            }),
            ..Default::default()
        };

        let (ai, player) = start_decks_from_fight(&fight, 7, None);

        assert_eq!(player.len(), 5);
        assert!(player.iter().any(|card| card.uid == Some(10)));
        assert!(player.iter().any(|card| card.uid == Some(11)));
        assert_eq!(ai.len(), 1);
        assert_eq!(ai[0].uid, Some(-1));
        assert_eq!(ai[0].skill_id, Some(301));
        assert!(matches!(ai[0].target_uid, Some(10 | 11)));
    }

    #[test]
    fn normal_hand_size_is_two_cards_per_character_plus_one_capped_at_eight() {
        let fight = |characters| Fight {
            attacker: Some(FightTeam {
                entitys: (0..characters)
                    .map(|index| {
                        entity(
                            index + 1,
                            1000 + index as i32,
                            index as i32 + 1,
                            &[101],
                            &[201],
                        )
                    })
                    .collect(),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(hand_size(&fight(0)), 0);
        assert_eq!(hand_size(&fight(1)), 3);
        assert_eq!(hand_size(&fight(2)), 5);
        assert_eq!(hand_size(&fight(3)), 7);
        assert_eq!(hand_size(&fight(4)), 8);
    }

    #[test]
    fn captured_start_decks_select_only_configured_candidates() {
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![entity(12, 1002, 1, &[202], &[203])],
                ..Default::default()
            }),
            defender: Some(FightTeam {
                entitys: vec![entity(-2, 2002, 1, &[302], &[303])],
                ..Default::default()
            }),
            ..Default::default()
        };
        let captured = (
            vec![
                CardInfo {
                    uid: Some(-2),
                    skill_id: Some(302),
                    card_effect: Some(999),
                    target_uid: Some(12),
                    ..Default::default()
                },
                CardInfo {
                    uid: Some(-2),
                    skill_id: Some(303),
                    target_uid: Some(999),
                    ..Default::default()
                },
                CardInfo {
                    uid: Some(-2),
                    skill_id: Some(999),
                    ..Default::default()
                },
            ],
            vec![CardInfo {
                uid: Some(12),
                skill_id: Some(202),
                card_effect: Some(999),
                ..Default::default()
            }],
        );

        let (ai, player) = start_decks_from_fight(&fight, 0, Some(captured));

        assert_eq!(ai[0].skill_id, Some(302));
        assert_eq!(ai[0].card_effect, None);
        assert_eq!(ai[0].target_uid, Some(12));
        assert_eq!(ai[1].skill_id, Some(303));
        assert_eq!(ai[1].target_uid, Some(0));
        assert_eq!(ai.len(), 2);
        assert_eq!(player[0].skill_id, Some(202));
        assert_eq!(player[0].card_effect, None);
    }

    #[test]
    fn draw_bag_keeps_sixteen_balanced_rank_one_cards_per_hero() {
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![entity(10, 1001, 1, &[101], &[201])],
                ..Default::default()
            }),
            ..Default::default()
        };

        let bag = draw_bag(&fight);

        assert_eq!(bag.len(), 16);
        assert_eq!(
            bag.iter().filter(|card| card.skill_id == Some(101)).count(),
            8
        );
        assert_eq!(
            bag.iter().filter(|card| card.skill_id == Some(201)).count(),
            8
        );
    }

    #[test]
    fn configured_device_cards_extend_the_draw_bag_without_inflating_the_normal_deck() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![entity(10, 3149, 1, &[31490111], &[31490131])],
                ..Default::default()
            }),
            ..Default::default()
        };

        let bag = draw_bag(&fight);

        assert_eq!(deck_size(&fight), 16);
        assert_eq!(bag.len(), 26);
        assert_eq!(
            bag.iter()
                .filter(|card| card.skill_id == Some(31446011))
                .count(),
            2
        );
        assert_eq!(
            bag.iter()
                .filter(|card| card.skill_id == Some(31490201))
                .count(),
            1
        );
    }

    fn entity(
        uid: i64,
        model_id: i32,
        position: i32,
        skill_group1: &[i32],
        skill_group2: &[i32],
    ) -> FightEntityInfo {
        FightEntityInfo {
            uid: Some(uid),
            model_id: Some(model_id),
            position: Some(position),
            current_hp: Some(100),
            skill_group1: skill_group1.to_vec(),
            skill_group2: skill_group2.to_vec(),
            ..Default::default()
        }
    }
}
