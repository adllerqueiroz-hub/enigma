use super::*;

#[test]
fn ezio_cloth_choice_runs_the_configured_skill_and_advances_qte_state() {
    crate::test_support::init_config();
    let entity = |uid, team_type, hp, attack| FightEntityInfo {
        uid: Some(uid),
        team_type: Some(team_type),
        current_hp: Some(hp),
        ex_point_type: Some(if uid > 0 { 2 } else { 0 }),
        attr: Some(sonettobuf::HeroAttribute {
            hp: Some(hp),
            attack: Some(attack),
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut runtime = BattleRuntime::new(Fight {
        version: Some(6),
        attacker: Some(FightTeam {
            entitys: vec![entity(10, 1, 1_000, 1_000)],
            ..Default::default()
        }),
        defender: Some(FightTeam {
            entitys: vec![entity(-1, 2, 100_000, 0)],
            ..Default::default()
        }),
        ..Default::default()
    });
    let origin = crate::engine::skill::rule::CommandOrigin {
        domain: crate::engine::skill::rule::RuleDomain::Behavior,
        key: crate::engine::skill::rule::DefinitionKey::new(100000, "EzioProps"),
    };
    runtime
        .managers
        .execute_ex_point(
            crate::engine::manager::ex_point::ExPointCommand::ConfigureSynchronization(
                crate::engine::manager::ex_point::ExPointConfigureSynchronization {
                    origin,
                    target_uid: 10,
                    definition: crate::engine::manager::ex_point::SynchronizationDefinition::new(
                        [312301323, 312301333, 312301343],
                        4,
                        100,
                    )
                    .unwrap(),
                },
            ),
        )
        .unwrap();
    runtime
        .managers
        .execute_buff(crate::engine::manager::buff::BuffCommand::Grant(
            crate::engine::manager::buff::BuffGrant {
                origin,
                source_uid: 10,
                target_uid: 10,
                buff_id: 229100,
                amount: None,
                occurrences: 1,
                child_uid_reservations: 0,
            },
        ))
        .unwrap();
    runtime.catalog = SkillEffectCatalog::from_roots(
        config::configs::get(),
        [312301323, 312301333, 312301343],
        [],
    );

    let reply = runtime
        .use_cloth_skill(UseClothSkillRequest {
            skill_id: Some(1),
            from_id: Some(10),
            to_id: Some(-1),
            r#type: Some(ClothSkillType::EzioBigSkill as i32),
        })
        .unwrap();

    let progress = runtime
        .managers
        .ex_point
        .synchronization_progress(10)
        .unwrap();
    assert_eq!((progress.completed_actions, progress.target_uid), (1, -1));
    assert!(progress.total_damage > 0);
    assert!(
        reply
            .round
            .unwrap()
            .fight_step
            .iter()
            .flat_map(|step| &step.act_effect)
            .any(|effect| {
                effect.buff_act_info.as_ref().is_some_and(|info| {
                    info.act_id == Some(10000)
                        && info.param.first() == Some(&2)
                        && info.param.get(2) == Some(&-1)
                })
            })
    );

    runtime
        .managers
        .hp
        .execute_command(crate::engine::manager::hp::HpCommand::Kill(
            crate::engine::manager::hp::HpKill {
                origin,
                source_uid: 10,
                target_uid: -1,
                config_effect: 60019,
            },
        ))
        .unwrap();
    assert!(
        runtime
            .use_cloth_skill(UseClothSkillRequest {
                skill_id: Some(1),
                from_id: Some(10),
                to_id: Some(-1),
                r#type: Some(ClothSkillType::EzioBigSkill as i32),
            })
            .is_none()
    );
}
