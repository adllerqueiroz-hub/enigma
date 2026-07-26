use crate::engine::{
    event::{kind::EventKind, payload::BattleEvent},
    manager::{
        buff::{ActiveBuffFeature, BuffCommand, BuffRemove, BuffRemoveSelector},
        card::{CardAddTemporary, CardCommand},
    },
    skill::{
        rule::output::{BattleCommand, RuleOp},
        subscriber::BuffActSubscriber,
    },
};

use super::{is_kind, registry::BuffActKind};

pub fn skill_id(feature: &ActiveBuffFeature) -> Option<i32> {
    if !is_kind(feature, BuffActKind::AddSpTempCard) {
        return None;
    }
    let [_, skill_id, ..] = feature.values.as_slice() else {
        return None;
    };
    (*skill_id > 0).then_some(*skill_id)
}

pub fn rule_op(feature: &ActiveBuffFeature, target_uid: i64, reserve_id: i64) -> Option<RuleOp> {
    Some(RuleOp::Command(BattleCommand::Card(
        CardCommand::AddTemporary(CardAddTemporary {
            origin: super::feature_command_origin(feature)?,
            target_uid,
            skill_id: skill_id(feature)?,
            reserve_id,
            team_type: feature.team_type,
        }),
    )))
}

pub fn subscriber_rule_ops(
    subscriber: &BuffActSubscriber,
    event: &BattleEvent,
    reserve_id: i64,
) -> Option<Vec<super::BuffActRuleOp>> {
    if !super::subscriber_is_kind(subscriber, BuffActKind::AddSpTempCard)
        || !matches!(event, BattleEvent::Kind(EventKind::RoundStartCard))
    {
        return None;
    }
    let [skill_id, ..] = subscriber.args.as_slice() else {
        return None;
    };
    if *skill_id <= 0 || reserve_id <= 0 {
        return None;
    }
    let origin = super::command_origin(subscriber)?;
    Some(vec![
        super::BuffActRuleOp::subscriber_from_owner(RuleOp::Command(BattleCommand::Card(
            CardCommand::AddTemporary(CardAddTemporary {
                origin,
                target_uid: subscriber.owner_uid,
                skill_id: *skill_id,
                reserve_id,
                team_type: subscriber.team_type,
            }),
        ))),
        super::BuffActRuleOp::separate_independent_command(RuleOp::Command(BattleCommand::Buff(
            BuffCommand::RemoveAfterTrigger(BuffRemove {
                origin,
                target_uid: subscriber.owner_uid,
                selector: BuffRemoveSelector::Uid(subscriber.buff_uid),
            }),
        ))),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{event::subscription::SubscriptionKey, skill::rule::DefinitionKey};

    #[test]
    fn exact_feature_emits_a_configured_temporary_card_command() {
        let feature = ActiveBuffFeature {
            owner_uid: 10,
            source_uid: 10,
            buff_uid: 20,
            buff_id: 30,
            amount: 1,
            team_type: 1,
            owner_alive: true,
            act_type: "AddSpTempCard".to_owned(),
            effect_time: 0,
            effect_condition: 0,
            raw: "815#999".to_owned(),
            values: vec![815, 999],
        };

        assert!(matches!(
            rule_op(&feature, 10, 40),
            Some(RuleOp::Command(BattleCommand::Card(
                CardCommand::AddTemporary(CardAddTemporary {
                    target_uid: 10,
                    skill_id: 999,
                    reserve_id: 40,
                    team_type: 1,
                    ..
                })
            )))
        ));
    }

    #[test]
    fn round_start_card_consumes_the_buff_after_adding_its_configured_card() {
        let subscriber = BuffActSubscriber {
            owner_uid: 10,
            source_uid: 10,
            buff_uid: 20,
            buff_id: 30,
            team_type: 1,
            owner_alive: true,
            amount: 1,
            key: SubscriptionKey::new(
                EventKind::RoundStartCard,
                DefinitionKey::new(815, "AddSpTempCard"),
            ),
            act_type: "AddSpTempCard".to_owned(),
            effect_time: 105,
            effect_condition: 0,
            args: vec![999],
            raw: "815#999".to_owned(),
        };

        let ops = subscriber_rule_ops(
            &subscriber,
            &BattleEvent::Kind(EventKind::RoundStartCard),
            3114,
        )
        .unwrap();

        assert!(matches!(
            &ops[0].op,
            RuleOp::Command(BattleCommand::Card(CardCommand::AddTemporary(add)))
                if add.skill_id == 999 && add.target_uid == 10 && add.reserve_id == 3114
        ));
        assert!(matches!(
            &ops[1].op,
            RuleOp::Command(BattleCommand::Buff(BuffCommand::RemoveAfterTrigger(remove)))
                if remove.target_uid == 10
                    && remove.selector == BuffRemoveSelector::Uid(20)
        ));
        assert_eq!(
            ops[0].scope,
            super::super::BuffActFrameScope::SubscriberFrame
        );
        assert_eq!(ops[0].source, super::super::BuffActFrameSource::Owner);
        assert!(!ops[1].group_with_siblings);
        assert_eq!(ops[1].frame_owner, super::super::BuffActFrameOwner::Command);
    }
}
