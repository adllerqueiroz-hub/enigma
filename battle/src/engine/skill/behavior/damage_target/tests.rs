use super::*;
use crate::engine::{
    manager::BattleManagers,
    runtime::determinism::RoundDeterminism,
    skill::{
        action::SkillModifiers,
        behavior::{self, classify::BehaviorSpec},
        effect::ParsedBehavior,
        target::{TargetContext, TargetPool},
    },
};

#[test]
fn configured_damage_target_is_recorded_once() {
    let managers = BattleManagers::default();
    let pool = TargetPool::default();
    let behavior =
        ParsedBehavior::from_spec(BehaviorSpec::new(60082, "Redirect"), Vec::new(), Vec::new());
    let mut determinism = RoundDeterminism::default();
    let mut modifiers = SkillModifiers::default();
    let mut target = TargetContext::default();

    for _ in 0..2 {
        let ops = behavior::rule_ops(
            BehaviorOpContext {
                source_uid: 10,
                source_team: 1,
                target_uid: -1,
                active_skill_id: 100,
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
        assert!(ops.is_empty());
    }

    assert_eq!(modifiers.redirected_damage_targets, vec![-1]);
    assert!(behavior::registry::find_key(60082, "Redirect").is_some());
    assert!(behavior::registry::find_key(60082, "Damage").is_none());
}
