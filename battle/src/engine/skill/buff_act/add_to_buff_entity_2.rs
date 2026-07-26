use crate::engine::{
    event::payload::BattleEvent,
    manager::buff::{BuffCommand, BuffGrant},
    skill::{
        action::SkillPhase,
        rule::output::{BattleCommand, RuleOp},
        subscriber::BuffActSubscriber,
    },
};

use super::registry::BuffActKind;

pub fn rule_ops(subscriber: &BuffActSubscriber, event: &BattleEvent) -> Option<Vec<RuleOp>> {
    if !super::subscriber_is_kind(subscriber, BuffActKind::AddToBuffEntity2) {
        return None;
    }
    let BattleEvent::SkillAction(action) = event else {
        return None;
    };
    if action.phase != SkillPhase::Immediate || action.source_uid != subscriber.owner_uid {
        return None;
    }
    let [buff_id, layer] = subscriber.args.as_slice() else {
        return None;
    };
    if *buff_id <= 0 || *layer <= 0 {
        return None;
    }
    Some(vec![RuleOp::Command(BattleCommand::Buff(
        BuffCommand::Grant(BuffGrant {
            origin: super::command_origin(subscriber)?,
            source_uid: if subscriber.source_uid != 0 {
                subscriber.source_uid
            } else {
                subscriber.owner_uid
            },
            target_uid: subscriber.owner_uid,
            buff_id: *buff_id,
            amount: Some(*layer),
            occurrences: 1,
            child_uid_reservations: 0,
        }),
    ))])
}

pub fn supports(args: &[i32]) -> bool {
    matches!(args, [buff_id, layer] if *buff_id > 0 && *layer > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn charged_ammunition_shape_is_an_owned_buff_grant() {
        assert!(supports(&[90071, 5]));
        assert!(!supports(&[90071]));
    }
}
