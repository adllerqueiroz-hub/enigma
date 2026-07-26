use super::*;

#[test]
fn remove_id_or_type_resolves_a_configured_type_id() {
    crate::test_support::init_config();
    let fight = Fight {
        attacker: Some(FightTeam {
            entitys: vec![FightEntityInfo {
                uid: Some(10),
                current_hp: Some(100),
                buffs: vec![BuffInfo {
                    uid: Some(2),
                    buff_id: Some(30482),
                    from_uid: Some(10),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut manager = BuffManager::default();
    manager.seed(&fight);

    let changes = manager
        .execute(
            &HpManager::default(),
            BuffCommand::Remove(BuffRemove {
                origin: CommandOrigin {
                    domain: RuleDomain::Behavior,
                    key: DefinitionKey::new(60010, "DisperseForce2"),
                },
                target_uid: 10,
                selector: BuffRemoveSelector::IdOrType(8112),
            }),
        )
        .unwrap();

    assert_eq!(changes.change.removed[0].buff.buff_id, Some(30482));
    assert!(manager.snapshot(10, 2).is_none());
}

#[test]
fn convert_consumes_one_exact_source_buff_before_granting_output() {
    crate::test_support::init_config();
    let entity = |uid, buffs| FightEntityInfo {
        uid: Some(uid),
        team_type: Some(1),
        current_hp: Some(100),
        buffs,
        ..Default::default()
    };
    let fight = Fight {
        attacker: Some(FightTeam {
            entitys: vec![
                entity(
                    10,
                    vec![BuffInfo {
                        uid: Some(1_000_001),
                        buff_id: Some(31020111),
                        count: Some(1),
                        ..Default::default()
                    }],
                ),
                entity(20, Vec::new()),
                entity(30, Vec::new()),
            ],
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut hp = HpManager::default();
    hp.seed(&fight);
    let mut manager = BuffManager::default();
    manager.seed(&fight);
    let origin = CommandOrigin {
        domain: RuleDomain::Behavior,
        key: DefinitionKey::new(60085, "DistributeBuff"),
    };
    let command = |target_uid| {
        BuffCommand::Convert(BuffConvert {
            origin,
            source_uid: 10,
            target_uid,
            source_buff_id: 31020111,
            output_buff_id: 31020118,
        })
    };

    let first = manager.execute(&hp, command(20)).unwrap();
    assert_eq!(first.change.removed.len(), 1);
    assert_eq!(
        first.change.added.as_ref().map(|added| added.target_uid),
        Some(20)
    );
    let exhausted = manager.execute(&hp, command(30)).unwrap();
    assert!(exhausted.change.removed.is_empty());
    assert!(exhausted.change.refreshed.is_empty());
    assert!(exhausted.change.added.is_none());
}

#[test]
fn duration_advance_uses_its_exact_effect_time_command() {
    crate::test_support::init_config();
    let fight = Fight {
        defender: Some(FightTeam {
            entitys: vec![FightEntityInfo {
                uid: Some(-1),
                current_hp: Some(100),
                buffs: vec![BuffInfo {
                    uid: Some(-10),
                    buff_id: Some(530000414),
                    duration: Some(2),
                    from_uid: Some(-1),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut manager = BuffManager::default();
    manager.seed(&fight);
    let advance = BuffDurationAdvance::new(
        crate::engine::skill::buff_act::effect_time::ROUND_END_ENTITY_SETTLEMENT,
        vec![-1],
        None,
    )
    .unwrap();

    let changes = manager
        .execute(&HpManager::default(), BuffCommand::AdvanceDuration(advance))
        .unwrap();

    assert_eq!(changes.origin.domain, RuleDomain::EffectTime);
    assert_eq!(changes.change.refreshed[0].after.duration, Some(1));
}

#[test]
fn duration_advance_does_not_echo_a_layered_buffs_previous_snapshot() {
    crate::test_support::init_config();
    let fight = Fight {
        attacker: Some(FightTeam {
            entitys: vec![FightEntityInfo {
                uid: Some(10),
                current_hp: Some(100),
                buffs: vec![BuffInfo {
                    uid: Some(20),
                    buff_id: Some(31200142),
                    duration: Some(3),
                    layer: Some(2),
                    from_uid: Some(10),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut manager = BuffManager::default();
    manager.seed(&fight);
    let advance = BuffDurationAdvance::new(
        crate::engine::skill::buff_act::effect_time::ROUND_START_DURATION,
        vec![10],
        None,
    )
    .unwrap();

    let changes = manager
        .execute(&HpManager::default(), BuffCommand::AdvanceDuration(advance))
        .unwrap();

    assert_eq!(changes.change.refreshed[0].before.duration, Some(3));
    assert_eq!(changes.change.refreshed[0].after.duration, Some(2));
    assert!(!changes.refresh_wire[0].echo_before);
}

#[test]
fn duration_advance_preserves_owner_order_across_refresh_and_expiry() {
    crate::test_support::init_config();
    let fight = Fight {
        attacker: Some(FightTeam {
            entitys: vec![
                FightEntityInfo {
                    uid: Some(10),
                    current_hp: Some(100),
                    buffs: vec![BuffInfo {
                        uid: Some(101),
                        buff_id: Some(530000112),
                        duration: Some(2),
                        from_uid: Some(-1),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                FightEntityInfo {
                    uid: Some(20),
                    current_hp: Some(100),
                    buffs: vec![BuffInfo {
                        uid: Some(99),
                        buff_id: Some(530000412),
                        duration: Some(1),
                        from_uid: Some(-1),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut manager = BuffManager::default();
    manager.seed(&fight);
    let advance = BuffDurationAdvance::new(
        crate::engine::skill::buff_act::effect_time::ROUND_END_ENTITY_SETTLEMENT,
        vec![10, 20],
        None,
    )
    .unwrap();

    let changes = manager
        .execute(&HpManager::default(), BuffCommand::AdvanceDuration(advance))
        .unwrap();

    assert!(matches!(
        changes.lifecycle_transitions.as_slice(),
        [
            BuffLifecycleTransition::Refreshed(refresh),
            BuffLifecycleTransition::Removed(removed),
        ] if refresh.target_uid == 10 && removed.target_uid == 20
    ));
}

#[test]
fn replacement_uses_the_exact_configured_output_without_inferring_a_family_variant() {
    crate::test_support::init_config();
    let fight = Fight {
        attacker: Some(FightTeam {
            entitys: vec![FightEntityInfo {
                uid: Some(10),
                current_hp: Some(100),
                buffs: vec![BuffInfo {
                    uid: Some(1),
                    buff_id: Some(30810305),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut manager = BuffManager::default();
    manager.seed(&fight);
    let hp = HpManager::default();
    let origin = CommandOrigin {
        domain: RuleDomain::BuffAct,
        key: DefinitionKey::new(1, "ReplaceBuff"),
    };

    let plan = manager
        .plan(
            &hp,
            BuffCommand::Replace(BuffReplace {
                origin,
                source_uid: 10,
                target_uid: 10,
                source: BuffSelector::IdOrType(30810101),
                replacement_id_or_type: 30810102,
            }),
        )
        .unwrap();
    assert!(manager.has_buff_id(10, 30810305));

    let changes = manager.commit(&hp, plan);
    assert_eq!(changes.origin, origin);
    assert_eq!(changes.change.removed[0].buff.buff_id, Some(30810305));
    assert_eq!(changes.change.added.unwrap().buff.buff_id, Some(30810102));
    assert!(!manager.has_buff_id(10, 30810305));
}

#[test]
fn stack_transition_is_part_of_the_grant_plan() {
    crate::test_support::init_config();
    let fight = Fight {
        attacker: Some(FightTeam {
            entitys: vec![FightEntityInfo {
                uid: Some(10),
                current_hp: Some(100),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let hp = HpManager::default();
    let mut manager = BuffManager::default();
    manager.seed(&fight);
    let changes = manager
        .execute(
            &hp,
            BuffCommand::Grant(BuffGrant {
                origin: CommandOrigin {
                    domain: RuleDomain::Behavior,
                    key: DefinitionKey::new(1, "AddBuff"),
                },
                source_uid: 10,
                target_uid: 10,
                buff_id: 5100,
                amount: Some(4),
                occurrences: 1,
                child_uid_reservations: 0,
            }),
        )
        .unwrap();

    assert!(!manager.has_buff_id(10, 5100));
    assert!(manager.has_buff_id(10, 5101));
    assert_eq!(changes.change.added.unwrap().buff.buff_id, Some(5101));
    assert!(
        changes
            .change
            .refreshed
            .iter()
            .all(|refresh| refresh.after.buff_id != Some(5100))
    );
}
