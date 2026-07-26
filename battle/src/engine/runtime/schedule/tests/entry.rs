use super::*;

#[test]
fn reserve_promotion_records_roster_change_and_removes_the_old_ai_cards() {
    init_config();
    let entity = |uid, hp, position| FightEntityInfo {
        uid: Some(uid),
        current_hp: Some(hp),
        position: Some(position),
        ..Default::default()
    };
    let mut fight = Fight {
        attacker: Some(FightTeam {
            entitys: vec![entity(10, 100, 1)],
            ..Default::default()
        }),
        defender: Some(FightTeam {
            entitys: vec![entity(-1, 100, 1)],
            sub_entitys: vec![entity(-2, 100, -1)],
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut managers = BattleManagers::seeded(&fight);
    managers.hp.lose(-1, 100, 10);
    let promotions = managers.promote_reserves(&mut fight);
    managers
        .execute_card(CardCommand::SetAiQueue(
            crate::engine::manager::card::CardSetAiQueue {
                origin: CommandOrigin {
                    domain: RuleDomain::Lifecycle,
                    key: DefinitionKey::new(0, "TestAiQueue"),
                },
                cards: vec![CardInfo {
                    uid: Some(-1),
                    skill_id: Some(100),
                    ..Default::default()
                }],
            },
        ))
        .unwrap();

    let result = run_promotions(
        &fight,
        &mut managers,
        &SkillEffectCatalog::default(),
        &mut RoundDeterminism::default(),
        TargetContext::default(),
        promotions,
    )
    .unwrap();
    let steps = crate::engine::packet::timeline::project(&result.frames).unwrap();

    assert!(managers.card.ai_queue().is_empty());
    assert_eq!(steps.len(), 1);
    assert_eq!(
        steps[0]
            .act_effect
            .iter()
            .map(|effect| effect.effect_type.unwrap())
            .collect::<Vec<_>>(),
        vec![
            sonettobuf::effect_type_enum::EffectType::Removeentitycards as i32,
            sonettobuf::effect_type_enum::EffectType::Changehero as i32,
        ]
    );
    assert_eq!(
        steps[0].act_effect[1].entity.as_ref().unwrap().uid,
        Some(-2)
    );
}

#[test]
fn promoted_defender_joins_the_normal_round_start_once() {
    init_config();
    let entity = |uid, hp, position, passive_skill| FightEntityInfo {
        uid: Some(uid),
        current_hp: Some(hp),
        ex_point: Some(0),
        position: Some(position),
        passive_skill,
        ..Default::default()
    };
    let mut fight = Fight {
        defender: Some(FightTeam {
            entitys: vec![entity(-1, 100, 1, Vec::new()), entity(-2, 0, 2, Vec::new())],
            sub_entitys: vec![entity(-3, 100, -1, vec![40])],
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut managers = BattleManagers::seeded(&fight);
    let promotions = managers.promote_reserves(&mut fight);
    managers.sync_roster(&fight);
    let pool = TargetPool::from_fight(&fight);
    let mut slot = SkillEffectSlot::new(
        ParsedBehavior::from_spec(BehaviorSpec::new(20002, "AddExPoint"), vec![1], Vec::new()),
        TargetRequest::self_only(),
    );
    slot.conditions = vec![ParsedCondition {
        opcode: 103,
        type_name: "None".to_owned(),
        kind: ParsedConditionKind::None(NoneMode::RoundStart),
        raw_args: Vec::new(),
    }];
    slot.compiled_route = ConditionRoute::compile(&slot.conditions);
    let mut catalog = SkillEffectCatalog::default();
    catalog.insert(ParsedSkillEffect {
        skill_id: 40,
        slots: vec![slot],
    });
    let mut determinism = RoundDeterminism::default();
    let context = TargetContext {
        current_round: 2,
        ..Default::default()
    };

    run_promotions(
        &fight,
        &mut managers,
        &catalog,
        &mut determinism,
        context,
        promotions,
    )
    .unwrap();
    run_before_ai_round_start(&mut managers, &pool, &catalog, &mut determinism, context, 2)
        .unwrap();

    assert_eq!(managers.ex_point.get(-3), 1);
}

#[test]
fn wave_entry_setup_runs_only_enter_fight() {
    init_config();
    let fight = Fight {
        defender: Some(FightTeam {
            entitys: vec![FightEntityInfo {
                uid: Some(-3),
                current_hp: Some(100),
                passive_skill: vec![2531, 2370, 2524, 2533],
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };
    let pool = TargetPool::from_fight(&fight);
    let mut managers = BattleManagers::seeded(&fight);
    let catalog = SkillEffectCatalog::from_fight(config::configs::get(), &fight);

    let result = run_wave_entry_setup(
        &mut managers,
        &pool,
        &catalog,
        &mut RoundDeterminism::default(),
        TargetContext {
            current_round: 2,
            ..Default::default()
        },
        &[-3],
    )
    .unwrap();

    fn collect_act_ids(step: &sonettobuf::FightStep, ids: &mut Vec<i32>) {
        if let Some(skill_id) = step.act_id.filter(|skill_id| *skill_id > 0) {
            ids.push(skill_id);
        }
        for child in step
            .act_effect
            .iter()
            .filter_map(|effect| effect.fight_step.as_ref())
        {
            collect_act_ids(child, ids);
        }
    }

    let steps = crate::engine::packet::timeline::project(&result.frames).unwrap();
    let mut act_ids = Vec::new();
    for step in &steps {
        collect_act_ids(step, &mut act_ids);
    }
    assert_eq!(steps.len(), 1);
    assert!(act_ids.contains(&2531));
    assert!(act_ids.contains(&2370));
    assert!(!act_ids.contains(&2524));
    assert!(!act_ids.contains(&2533));
    assert!(matches!(
        result.frames[0].owner,
        FrameOwner::RoundPhase(RoundPhase::EntityEntrySetup)
    ));
}
