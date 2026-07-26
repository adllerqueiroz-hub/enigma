use super::{SetupFrameContainer, context_for_setup_stage};
use crate::engine::skill::{
    condition::registry::SetupFrameScope, rule::SetupStage, target::TargetContext,
};

#[test]
fn round_transition_conditions_read_the_completed_round() {
    let context = context_for_setup_stage(
        TargetContext {
            current_round: 2,
            ..Default::default()
        },
        SetupStage::RoundTransitionStart,
    );

    assert_eq!(context.current_round, 1);
    assert_eq!(
        context_for_setup_stage(context, SetupStage::RoundStart).current_round,
        1
    );
}

#[test]
fn round_phase_owns_entity_scope_without_changing_registry_scope() {
    assert!(!SetupFrameContainer::Standalone.owns_entity_scope(Some(SetupFrameScope::Entity), 10));
    assert!(SetupFrameContainer::Standalone.owns_entity_scope(Some(SetupFrameScope::Side), 10));
    assert!(SetupFrameContainer::RoundPhase.owns_entity_scope(Some(SetupFrameScope::Entity), 10));
}

#[test]
fn opening_round_phase_roots_entity_scope() {
    assert!(
        SetupFrameContainer::OpeningRoundPhase
            .roots_entity_scope(Some(SetupFrameScope::Entity), 10)
    );
    assert!(
        !SetupFrameContainer::OpeningRoundPhase.roots_entity_scope(Some(SetupFrameScope::Side), 10)
    );
}

#[test]
fn side_rule_owner_uses_the_setup_side_frame() {
    assert!(SetupFrameContainer::Standalone.owns_entity_scope(
        Some(SetupFrameScope::Entity),
        crate::engine::fight::rules::DEFENDER_SIDE_UID,
    ));
}
