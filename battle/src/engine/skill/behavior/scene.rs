use crate::engine::skill::{
    behavior::{BehaviorOpContext, classify::BehaviorKind, registry::BehaviorHandler},
    effect::ParsedBehavior,
    rule::output::RuleOp,
};

pub struct Handler;

impl BehaviorHandler for Handler {
    const VALIDATES_ARGUMENTS: bool = true;

    fn supports(behavior: &ParsedBehavior) -> bool {
        matches!(
            (behavior.spec.kind, behavior.args.as_slice()),
            (BehaviorKind::ChangeScene, [scene_id]) if *scene_id > 0
        )
    }

    fn emit_ops(_: BehaviorOpContext<'_>, behavior: &ParsedBehavior) -> Option<Vec<RuleOp>> {
        Self::supports(behavior).then(|| {
            vec![RuleOp::SceneChange {
                scene_id: behavior.args[0],
            }]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_change_requires_one_configured_scene() {
        let valid = ParsedBehavior::new(60268, "ChangeScene", vec![14501]);
        let invalid = ParsedBehavior::new(60268, "ChangeScene", vec![]);

        assert!(Handler::supports(&valid));
        assert!(!Handler::supports(&invalid));
    }
}
