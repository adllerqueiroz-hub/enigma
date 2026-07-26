use super::*;

#[test]
fn card_consumption_projects_its_owned_wire_effect() {
    let mut cards = CardManager::new(vec![sonettobuf::CardInfo {
        uid: Some(10),
        skill_id: Some(100),
        card_effect: Some(1),
        ..Default::default()
    }]);
    cards.seed(&sonettobuf::Fight {
        attacker: Some(sonettobuf::FightTeam {
            entitys: vec![sonettobuf::FightEntityInfo {
                uid: Some(10),
                skill_group1: vec![100, 101, 102],
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    });
    let changes = cards
        .execute_command(CardCommand::ConsumeForEffect(
            crate::engine::manager::card::CardConsumeForEffect {
                origin: CommandOrigin {
                    domain: RuleDomain::Behavior,
                    key: DefinitionKey::new(60222, "ConsumeCardAddBuff"),
                },
                owner_uid: 10,
                indices: vec![0],
            },
        ))
        .unwrap();

    let effects = project_change_for_test(&BattleChange::Card(Box::new(changes))).unwrap();

    assert_eq!(effects.len(), 1);
    assert_eq!(effects[0].effect_type, Some(EffectType::Cardremove as i32));
    assert_eq!(effects[0].reserve_str.as_deref(), Some("1"));
}

#[test]
fn hand_rank_change_projects_the_committed_card_and_resource_state() {
    use crate::engine::manager::{
        card::{CardSetup, HandCardRankUp},
        eureka::{EUREKA_RESOURCE_ID, EurekaChange, EurekaCommand},
    };

    crate::test_support::init_config();
    let origin = CommandOrigin {
        domain: RuleDomain::Behavior,
        key: DefinitionKey::new(50034, "ConsumePowerUpgradeSkillCard"),
    };
    let fight = Fight {
        attacker: Some(FightTeam {
            entitys: vec![FightEntityInfo {
                uid: Some(10),
                team_type: Some(1),
                current_hp: Some(100),
                skill_group1: vec![30650221, 30650222, 30650223],
                power_infos: vec![sonettobuf::PowerInfo {
                    power_id: Some(EUREKA_RESOURCE_ID),
                    num: Some(2),
                    max: Some(6),
                }],
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut managers = BattleManagers::seeded(&fight);
    managers
        .execute_card(CardCommand::Setup(CardSetup {
            hand: vec![CardInfo {
                uid: Some(10),
                skill_id: Some(30650221),
                ..Default::default()
            }],
            draw_pile: Vec::new(),
            deck_num: 1,
        }))
        .unwrap();
    managers
        .execute_eureka(EurekaCommand::Change(EurekaChange {
            origin,
            source_uid: 10,
            target_uid: 10,
            power_id: EUREKA_RESOURCE_ID,
            delta: -2,
            effect_type: EffectType::Powerchange as i32,
        }))
        .unwrap();
    let changes = managers
        .execute_card(CardCommand::RankUpHand(HandCardRankUp {
            origin,
            owner_uid: 10,
            hand_index: 0,
        }))
        .unwrap();

    let effects = project_change_for_test(&BattleChange::Card(Box::new(changes))).unwrap();

    assert_eq!(effects.len(), 1);
    let effect = &effects[0];
    assert_eq!(effect.effect_type, Some(EffectType::Cardlevelchange as i32));
    assert_eq!(effect.target_id, Some(1));
    assert_eq!(effect.effect_num, Some(30650222));
    assert_eq!(effect.config_effect, Some(50034));
    assert_eq!(
        effect
            .entity
            .as_ref()
            .and_then(|entity| entity.power_infos.first())
            .and_then(|power| power.num),
        Some(0)
    );
}
