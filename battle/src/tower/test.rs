use database::models::game::tower::TowerType;
use sonettobuf::{Fight, FightEntityInfo, FightTeam, StartDungeonRequest, StartTowerBattleRequest};

use super::{fight, validate_battle_start};
use crate::dungeon::BuiltFight;

fn tables() -> &'static config::GameDB {
    crate::test_support::init_config();
    config::configs::get()
}

fn request(
    tower_type: TowerType,
    tower_id: i32,
    layer_id: i32,
    difficulty: i32,
    episode_id: i32,
) -> StartTowerBattleRequest {
    StartTowerBattleRequest {
        start_dungeon_request: Some(StartDungeonRequest {
            episode_id: Some(episode_id),
            ..Default::default()
        }),
        r#type: Some(tower_type.id()),
        tower_id: Some(tower_id),
        layer_id: Some(layer_id),
        difficulty: Some(difficulty),
        talent_plan_id: Some(0),
    }
}

#[test]
fn tower_episode_identity_is_derived_from_each_config_family() {
    let tables = tables();

    let permanent = tables.tower_permanent_episode.iter().next().unwrap();
    let permanent_episode = permanent
        .episode_ids
        .split('|')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    assert!(
        validate_battle_start(
            tables,
            &request(
                TowerType::Normal,
                0,
                permanent.layer_id,
                0,
                permanent_episode,
            ),
        )
        .is_ok()
    );

    let boss = tables.tower_boss_episode.iter().next().unwrap();
    assert!(
        validate_battle_start(
            tables,
            &request(
                TowerType::Boss,
                boss.tower_id,
                boss.layer_id,
                0,
                boss.episode_id,
            ),
        )
        .is_ok()
    );

    let teach = tables.tower_boss_teach.iter().next().unwrap();
    assert!(
        validate_battle_start(
            tables,
            &request(
                TowerType::Boss,
                teach.tower_id,
                0,
                teach.teach_id,
                teach.episode_id,
            ),
        )
        .is_ok()
    );

    let limited = tables.tower_limited_episode.iter().next().unwrap();
    assert!(
        validate_battle_start(
            tables,
            &request(
                TowerType::Limited,
                limited.season,
                limited.layer_id,
                limited.difficulty,
                limited.episode_id,
            ),
        )
        .is_ok()
    );
}

#[test]
fn related_but_mismatched_tower_coordinates_are_rejected() {
    let tables = tables();
    let boss = tables.tower_boss_episode.iter().next().unwrap();
    let mismatched = request(
        TowerType::Boss,
        boss.tower_id,
        boss.layer_id + 1,
        0,
        boss.episode_id,
    );

    assert!(validate_battle_start(tables, &mismatched).is_err());
}

#[test]
fn boss_five_setup_is_derived_from_the_captured_config_chain() {
    let tables = tables();
    let plan = tables
        .tower_talent_plan
        .iter()
        .find(|plan| plan.boss_id == 5 && plan.plan_id == 502)
        .unwrap();
    let talents = fight::system_plan_talents(tables, 5, 10, &plan.talent_ids);
    let mut built = BuiltFight {
        fight: Fight {
            attacker: Some(FightTeam {
                entitys: (1..=4)
                    .map(|career| FightEntityInfo {
                        uid: Some(career as i64),
                        level: Some(180),
                        career: Some(career),
                        ..Default::default()
                    })
                    .collect(),
                ..Default::default()
            }),
            ..Default::default()
        },
        ex_attributes: vec![],
        sp_attributes: vec![],
        battle_rule_skills: vec![],
    };

    fight::apply_assist_boss(tables, 401_299_742, 5, 10, &talents, &mut built).unwrap();

    let team = built.fight.attacker.unwrap();
    let boss = team.assist_boss.unwrap();
    assert_eq!(talents.len(), 15);
    assert_eq!(boss.uid, Some(-1));
    assert_eq!(boss.attr.unwrap().attack, Some(2380));
    assert_eq!(boss.power_infos[0].power_id, Some(4));
    assert_eq!(boss.passive_skill.len(), 14);
    assert_eq!(
        team.assist_boss_info.unwrap().skills[0].skill_id,
        Some(1251001)
    );
    assert!(team.entitys.iter().all(|hero| {
        [1252007, 1252008, 1252009, 123900605, 1259001, 1252001]
            .iter()
            .all(|skill| hero.passive_skill.contains(skill))
    }));
}

#[test]
fn tower_extra_rules_keep_virtual_and_entity_ownership_separate() {
    let tables = tables();
    let mut built = BuiltFight {
        fight: Fight {
            attacker: Some(FightTeam {
                entitys: (1..=4)
                    .map(|uid| FightEntityInfo {
                        uid: Some(uid),
                        level: Some(1),
                        ..Default::default()
                    })
                    .collect(),
                ..Default::default()
            }),
            defender: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(-2),
                    current_hp: Some(100),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        },
        ex_attributes: vec![],
        sp_attributes: vec![],
        battle_rule_skills: vec![],
    };

    fight::apply_assist_boss(tables, 1, 1, 5, &[101], &mut built).unwrap();

    assert_eq!(
        built.battle_rule_skills,
        vec![crate::engine::fight::rules::OwnedBattleSkill {
            owner_uid: crate::engine::fight::rules::ATTACKER_SIDE_UID,
            skill_id: 370002010,
        }]
    );
    assert!(
        !built
            .fight
            .attacker
            .unwrap()
            .assist_boss
            .unwrap()
            .passive_skill
            .contains(&370002010)
    );
    assert!(
        built.fight.defender.unwrap().entitys[0]
            .passive_skill
            .contains(&370001020)
    );
}
