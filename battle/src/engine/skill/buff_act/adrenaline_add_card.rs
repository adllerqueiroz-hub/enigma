use crate::engine::{
    event::{kind::EventKind, payload::BattleEvent},
    manager::{
        BattleManagers,
        card::{CardAddGenerated, CardCommand},
        ex_point::{ExPointCommand, ExPointSet},
    },
    skill::{
        buff_act::registry::BuffActKind,
        rule::output::{BattleCommand, RuleOp},
        subscriber::BuffActSubscriber,
    },
};

pub fn rule_ops(
    managers: &BattleManagers,
    subscriber: &BuffActSubscriber,
    event: &BattleEvent,
) -> Option<Vec<RuleOp>> {
    if !super::subscriber_is_kind(subscriber, BuffActKind::AdrenalineAddCard)
        || !matches!(event, BattleEvent::Kind(EventKind::RoundStartCard))
    {
        return None;
    }
    let (threshold, skill_id) = parse(&subscriber.args)?;
    if managers.ex_point.get(subscriber.owner_uid) < threshold {
        return Some(Vec::new());
    }
    let origin = super::command_origin(subscriber)?;
    Some(vec![
        RuleOp::Command(BattleCommand::ExPoint(ExPointCommand::Set(ExPointSet {
            origin,
            source_uid: subscriber.owner_uid,
            target_uid: subscriber.owner_uid,
            value: 0,
            config_effect: 0,
            effect_type: sonettobuf::effect_type_enum::EffectType::Expointchange as i32,
        }))),
        RuleOp::Command(BattleCommand::Card(CardCommand::AddGenerated(
            CardAddGenerated {
                origin,
                target_uid: subscriber.owner_uid,
                skill_id,
            },
        ))),
    ])
}

pub fn supports(args: &[i32]) -> bool {
    parse(args).is_some()
}

fn parse(args: &[i32]) -> Option<(i32, i32)> {
    let [threshold, skill_id] = args else {
        return None;
    };
    (*threshold > 0 && *skill_id > 0).then_some((*threshold, *skill_id))
}

#[cfg(test)]
mod tests {
    use sonettobuf::{BuffInfo, Fight, FightEntityInfo, FightTeam};

    use super::*;
    use crate::engine::{event::subscription::SubscriptionKey, skill::rule::DefinitionKey};

    #[test]
    fn below_threshold_does_not_add_the_configured_card() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(10),
                    current_hp: Some(100),
                    ex_point: Some(9),
                    ex_point_type: Some(3),
                    buffs: vec![BuffInfo {
                        uid: Some(20),
                        buff_id: Some(31242140),
                        from_uid: Some(10),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let subscriber = BuffActSubscriber {
            owner_uid: 10,
            source_uid: 10,
            buff_uid: 20,
            buff_id: 31242140,
            team_type: 1,
            owner_alive: true,
            amount: 1,
            key: SubscriptionKey::new(
                EventKind::RoundStartCard,
                DefinitionKey::new(10001, "AdrenalineAddCard"),
            ),
            act_type: "AdrenalineAddCard".to_owned(),
            effect_time: 105,
            effect_condition: 0,
            args: vec![10, 31242103],
            raw: "10001#10#31242103".to_owned(),
        };

        let ops = rule_ops(
            &BattleManagers::seeded(&fight),
            &subscriber,
            &BattleEvent::Kind(EventKind::RoundStartCard),
        )
        .unwrap();

        assert!(ops.is_empty());
        assert!(supports(&[10, 31242103]));
        assert!(!supports(&[10]));
        assert!(!supports(&[10, 31242103, 6, 31242102]));
    }

    #[test]
    fn terminal_threshold_resets_adrenaline_before_adding_the_card() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(10),
                    current_hp: Some(100),
                    ex_point: Some(10),
                    ex_point_type: Some(3),
                    buffs: vec![BuffInfo {
                        uid: Some(20),
                        buff_id: Some(31242140),
                        from_uid: Some(10),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let subscriber = BuffActSubscriber {
            owner_uid: 10,
            source_uid: 10,
            buff_uid: 20,
            buff_id: 31242140,
            team_type: 1,
            owner_alive: true,
            amount: 1,
            key: SubscriptionKey::new(
                EventKind::RoundStartCard,
                DefinitionKey::new(10001, "AdrenalineAddCard"),
            ),
            act_type: "AdrenalineAddCard".to_owned(),
            effect_time: 105,
            effect_condition: 0,
            args: vec![10, 31242103],
            raw: "10001#10#31242103".to_owned(),
        };

        let ops = rule_ops(
            &BattleManagers::seeded(&fight),
            &subscriber,
            &BattleEvent::Kind(EventKind::RoundStartCard),
        )
        .unwrap();

        assert!(matches!(
            ops.as_slice(),
            [
                RuleOp::Command(BattleCommand::ExPoint(ExPointCommand::Set(ExPointSet {
                    target_uid: 10,
                    value: 0,
                    ..
                }))),
                RuleOp::Command(BattleCommand::Card(CardCommand::AddGenerated(
                    CardAddGenerated {
                        target_uid: 10,
                        skill_id: 31242103,
                        ..
                    }
                )))
            ]
        ));
    }
}
