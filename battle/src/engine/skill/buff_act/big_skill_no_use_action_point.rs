use crate::engine::{
    event::payload::BattleEvent,
    manager::{
        BattleManagers,
        buff::{ActiveBuffFeature, BuffCommand, BuffRemove, BuffRemoveSelector},
    },
    skill::{
        effect::SkillEffectCatalog,
        rule::output::{BattleCommand, RuleOp},
        subscriber::BuffActSubscriber,
    },
};

use super::{is_kind, registry::BuffActKind};

pub fn supports(args: &[i32]) -> bool {
    args.is_empty()
}

pub fn skill_uses_action_point(
    features: &[ActiveBuffFeature],
    owner_uid: i64,
    is_big_skill: bool,
) -> bool {
    !is_big_skill
        || !features.iter().any(|feature| {
            feature.owner_uid == owner_uid && is_kind(feature, BuffActKind::BigSkillNoUseActPoint)
        })
}

pub fn consume_rule_op(managers: &BattleManagers, feature: &ActiveBuffFeature) -> Option<RuleOp> {
    managers
        .buff
        .snapshot(feature.owner_uid, feature.buff_uid)?;
    Some(RuleOp::Command(BattleCommand::Buff(BuffCommand::Remove(
        BuffRemove {
            origin: super::feature_command_origin(feature)?,
            target_uid: feature.owner_uid,
            selector: BuffRemoveSelector::Uid(feature.buff_uid),
        },
    ))))
}

pub fn rule_ops(
    managers: &BattleManagers,
    catalog: &SkillEffectCatalog,
    subscriber: &BuffActSubscriber,
    event: &BattleEvent,
) -> Option<Vec<RuleOp>> {
    let BattleEvent::AllyAction(action) = event else {
        return None;
    };
    if action.source_uid != subscriber.owner_uid || !catalog.is_big_skill(action.skill_id) {
        return Some(Vec::new());
    }
    let feature = managers
        .buff
        .active_features(&managers.hp)
        .into_iter()
        .find(|feature| feature.buff_uid == subscriber.buff_uid)?;
    consume_rule_op(managers, &feature).map(|op| vec![op])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{event::kind::EventKind, skill::action::ActionEvent};
    use sonettobuf::{BuffInfo, Fight, FightEntityInfo, FightTeam};

    fn feature(kind: &str) -> ActiveBuffFeature {
        ActiveBuffFeature {
            owner_uid: 1,
            source_uid: 1,
            buff_uid: 2,
            buff_id: 3,
            amount: 1,
            team_type: 1,
            owner_alive: true,
            act_type: kind.to_owned(),
            effect_time: 0,
            effect_condition: 0,
            raw: String::new(),
            values: vec![946],
        }
    }

    #[test]
    fn only_the_owner_ultimate_waives_the_action_point() {
        let feature = feature("BigSkillNoUseActPoint");
        assert!(!skill_uses_action_point(
            std::slice::from_ref(&feature),
            1,
            true
        ));
        assert!(skill_uses_action_point(
            std::slice::from_ref(&feature),
            1,
            false
        ));
        assert!(skill_uses_action_point(&[feature], 2, true));
    }

    #[test]
    fn completed_owner_ultimate_consumes_the_exact_buff_instance() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(10),
                    current_hp: Some(100),
                    buffs: vec![BuffInfo {
                        uid: Some(2),
                        buff_id: Some(31280116),
                        from_uid: Some(10),
                        count: Some(1),
                        layer: Some(1),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = BattleManagers::seeded(&fight);
        let features = managers.buff.active_features(&managers.hp);
        assert!(
            features
                .iter()
                .any(|feature| is_kind(feature, BuffActKind::BigSkillNoUseActPoint)),
            "features={features:?}"
        );
        let subscribers =
            crate::engine::skill::subscriber::for_active_buffs(&managers, EventKind::AllyAction);
        let [subscriber] = subscribers.as_slice() else {
            panic!("expected exact 946 subscriber")
        };
        let event = BattleEvent::AllyAction(ActionEvent {
            source_uid: 10,
            target_uid: -1,
            skill_id: 31280131,
            skill_slot: 3,
            is_attack: true,
            rank: 1,
            skill_type: 0,
            effect_tag: 1,
            additional_moxie: 0,
            extra_skill_kind: 0,
            assassinate: false,
            ..Default::default()
        });

        assert!(matches!(
            rule_ops(
                &managers,
                crate::engine::skill::effect::catalog::global(),
                subscriber,
                &event,
            )
            .unwrap()
            .as_slice(),
            [RuleOp::Command(BattleCommand::Buff(BuffCommand::Remove(
                BuffRemove {
                    target_uid: 10,
                    selector: BuffRemoveSelector::Uid(2),
                    ..
                }
            )))]
        ));
    }
}
