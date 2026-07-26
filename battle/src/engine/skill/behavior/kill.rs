use crate::engine::{
    manager::hp::{HpCommand, HpKill, HpManager},
    skill::{
        behavior::{BehaviorOpContext, classify::BehaviorKind, registry::BehaviorHandler},
        effect::ParsedBehavior,
        rule::output::{BattleCommand, RuleOp},
        target::{TargetRequest, TargetResolver},
    },
};

pub fn rule_op(
    source_uid: i64,
    target_uid: i64,
    behavior: &ParsedBehavior,
    hp: &HpManager,
) -> Option<RuleOp> {
    if behavior.spec.kind != BehaviorKind::Kill {
        return None;
    }
    let amount = hp.current(target_uid);
    if amount <= 0 {
        return None;
    }
    Some(RuleOp::Command(BattleCommand::Hp(HpCommand::Kill(
        HpKill {
            origin: super::command_origin(behavior)?,
            source_uid,
            target_uid,
            config_effect: behavior.spec.key.opcode,
        },
    ))))
}

pub(super) fn supports(behavior: &ParsedBehavior) -> bool {
    behavior.args.is_empty()
}

pub(super) struct Handler;

impl BehaviorHandler for Handler {
    fn emit_ops(context: BehaviorOpContext<'_>, behavior: &ParsedBehavior) -> Option<Vec<RuleOp>> {
        if behavior.spec.kind == BehaviorKind::KillTargets {
            let [0, target_code] = behavior.args.as_slice() else {
                return None;
            };
            let targets = TargetResolver::resolve_with_managers_and_context(
                &TargetRequest {
                    code: *target_code,
                    raw: Vec::new(),
                },
                context.active_skill_id,
                context.source_uid,
                context.pool,
                context.determinism,
                Some(context.managers),
                *context.target,
            );
            let origin = super::command_origin(behavior)?;
            return Some(
                targets
                    .into_iter()
                    .filter(|target_uid| context.managers.hp.current(*target_uid) > 0)
                    .map(|target_uid| {
                        RuleOp::Command(BattleCommand::Hp(HpCommand::Kill(HpKill {
                            origin,
                            source_uid: context.source_uid,
                            target_uid,
                            config_effect: behavior.spec.key.opcode,
                        })))
                    })
                    .collect(),
            );
        }
        if context.managers.hp.current(context.target_uid) <= 0 {
            return Some(Vec::new());
        }
        rule_op(
            context.source_uid,
            context.target_uid,
            behavior,
            &context.managers.hp,
        )
        .map(|op| vec![op])
    }
}

#[cfg(test)]
mod tests {
    use sonettobuf::{Fight, FightEntityInfo, FightTeam};

    use super::*;
    use crate::engine::manager::BattleManagers;
    use crate::engine::runtime::determinism::RoundDeterminism;
    use crate::engine::skill::target::TargetPool;

    #[test]
    fn kill_emits_a_semantic_kill_command() {
        let fight = Fight {
            defender: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(-1),
                    current_hp: Some(50),
                    shield_value: Some(100),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = BattleManagers::seeded(&fight);
        let behavior = ParsedBehavior::new(60015, "Kill", Vec::new());

        assert!(matches!(
            rule_op(10, -1, &behavior, &managers.hp),
            Some(RuleOp::Command(BattleCommand::Hp(HpCommand::Kill(
                HpKill {
                    source_uid: 10,
                    target_uid: -1,
                    ..
                }
            ))))
        ));
    }

    #[test]
    fn kill_targets_resolves_its_configured_selector_without_killing_the_caster() {
        let entity = |uid, hp| FightEntityInfo {
            uid: Some(uid),
            team_type: Some(1),
            current_hp: Some(hp),
            ..Default::default()
        };
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![
                    entity(10, 100),
                    entity(11, 100),
                    entity(12, 100),
                    entity(13, 0),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = BattleManagers::seeded(&fight);
        let pool = TargetPool::from_fight(&fight);
        let mut determinism = RoundDeterminism::default();
        let mut modifiers = crate::engine::skill::action::SkillModifiers::default();
        let mut target = crate::engine::skill::target::TargetContext::default();
        let behavior = ParsedBehavior::from_spec(
            crate::engine::skill::behavior::classify::BehaviorSpec::new(60019, "KillTargets"),
            vec![0, 102],
            vec!["0".into(), "102".into()],
        );

        let ops = Handler::emit_ops(
            BehaviorOpContext {
                source_uid: 10,
                source_team: 1,
                target_uid: 10,
                active_skill_id: 1,
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

        assert_eq!(
            ops.iter()
                .filter_map(|op| match op {
                    RuleOp::Command(BattleCommand::Hp(HpCommand::Kill(kill))) => {
                        Some(kill.target_uid)
                    }
                    _ => None,
                })
                .collect::<Vec<_>>(),
            vec![11, 12]
        );
    }

    #[test]
    fn killing_a_dead_target_is_a_valid_empty_operation() {
        let fight = Fight {
            defender: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(-1),
                    current_hp: Some(0),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = BattleManagers::seeded(&fight);
        let mut determinism = RoundDeterminism::default();
        let mut modifiers = crate::engine::skill::action::SkillModifiers::default();
        let mut target = crate::engine::skill::target::TargetContext::default();
        let behavior = ParsedBehavior::from_spec(
            crate::engine::skill::behavior::classify::BehaviorSpec::new(60015, "Kill"),
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(
            super::super::rule_ops(
                BehaviorOpContext {
                    source_uid: 10,
                    source_team: 1,
                    target_uid: -1,
                    active_skill_id: 0,
                    transfer_count: 1,
                    event: None,
                    managers: &managers,
                    pool: &TargetPool::from_fight(&fight),
                    determinism: &mut determinism,
                    modifiers: &mut modifiers,
                    target: &mut target,
                },
                &behavior,
            ),
            Some(Vec::new())
        );
    }
}
