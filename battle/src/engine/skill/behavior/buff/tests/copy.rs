use super::*;

#[test]
fn status_copy_uses_the_event_owner_snapshot_and_registered_target() {
    crate::test_support::init_config();
    let fight = Fight {
        defender: Some(FightTeam {
            entitys: vec![
                FightEntityInfo {
                    uid: Some(-3),
                    team_type: Some(2),
                    current_hp: Some(0),
                    buffs: vec![
                        BuffInfo {
                            buff_id: Some(304),
                            uid: Some(20),
                            count: Some(2),
                            ..Default::default()
                        },
                        BuffInfo {
                            buff_id: Some(4010),
                            uid: Some(21),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                FightEntityInfo {
                    uid: Some(-2),
                    team_type: Some(2),
                    current_hp: Some(100),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }),
        ..Default::default()
    };
    let managers = BattleManagers::seeded(&fight);
    let pool = TargetPool::from_fight(&fight);
    let behavior = ParsedBehavior::from_spec(
        crate::engine::skill::behavior::classify::BehaviorSpec::new(60117, "SelfRandomCopyBuffs"),
        vec![1, BuffStatus::NegativeStatus as i32],
        Vec::new(),
    );
    let mut determinism = RoundDeterminism::default();
    determinism.enqueue_random_buffs([304]);
    let mut modifiers = crate::engine::skill::action::SkillModifiers::default();
    let mut target = crate::engine::skill::target::TargetContext::default();

    let ops = super::super::super::rule_ops(
        BehaviorOpContext {
            source_uid: -3,
            source_team: 2,
            target_uid: -2,
            active_skill_id: 370001020,
            transfer_count: 1,
            event: None,
            managers: &managers,
            pool: &pool,
            determinism: &mut determinism,
            modifiers: &mut modifiers,
            target: &mut target,
        },
        &behavior,
    )
    .unwrap();

    assert!(matches!(
        ops.as_slice(),
        [RuleOp::Command(BattleCommand::Buff(BuffCommand::Grant(
            BuffGrant {
                source_uid: -3,
                target_uid: -2,
                buff_id: 304,
                amount: Some(2),
                ..
            }
        )))]
    ));
}
