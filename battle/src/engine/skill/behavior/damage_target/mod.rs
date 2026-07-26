use crate::engine::skill::{
    behavior::{BehaviorOpContext, registry::BehaviorHandler},
    effect::ParsedBehavior,
    rule::output::RuleOp,
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
        if !context
            .modifiers
            .redirected_damage_targets
            .contains(&context.target_uid)
        {
            context
                .modifiers
                .redirected_damage_targets
                .push(context.target_uid);
        }
        Some(Vec::new())
    }
}

#[cfg(test)]
mod tests;
