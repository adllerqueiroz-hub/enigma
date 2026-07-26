use crate::engine::{
    manager::entity::EntitySkillCommand,
    skill::{
        behavior::{BehaviorOpContext, registry::BehaviorHandler},
        condition::extra::ExtraSkillKind,
        effect::ParsedBehavior,
        rule::output::{BattleCommand, RuleOp},
    },
};

pub(super) struct Handler;

impl BehaviorHandler for Handler {
    const VALIDATES_ARGUMENTS: bool = true;

    fn supports(behavior: &ParsedBehavior) -> bool {
        behavior.args.is_empty()
    }

    fn emit_ops(context: BehaviorOpContext<'_>, behavior: &ParsedBehavior) -> Option<Vec<RuleOp>> {
        if !Self::supports(behavior) {
            return None;
        }
        Some(vec![RuleOp::Command(BattleCommand::EntitySkill(
            EntitySkillCommand {
                origin: super::command_origin(behavior)?,
                target_uid: context.target_uid,
                ultimate_kind: ExtraSkillKind::ExtraAction,
            },
        ))])
    }
}
