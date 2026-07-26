use crate::engine::{
    event::payload::BattleEvent,
    manager::{
        BattleManagers,
        buff::{BuffCommand, BuffGrant},
        ex_point::ExPointKind,
    },
    skill::{
        buff_act::{BuffActRuleOp, registry::BuffActKind},
        rule::output::{BattleCommand, RuleOp},
        subscriber::BuffActSubscriber,
        target::TargetPool,
    },
};

pub fn supports(args: &[i32]) -> bool {
    matches!(args, [buff_id, transfer_mode, threshold, limit]
        if *buff_id > 0 && *transfer_mode > 0 && *threshold >= 0 && *limit > 0)
}

pub fn referenced_buff(args: &[i32]) -> Option<i32> {
    supports(args).then(|| args[0])
}

pub fn rule_ops(
    managers: &BattleManagers,
    pool: &TargetPool,
    subscriber: &BuffActSubscriber,
    event: &BattleEvent,
) -> Option<Vec<BuffActRuleOp>> {
    let BattleEvent::ExPointOverflow(change) = event else {
        return Some(Vec::new());
    };
    let [buff_id, transfer_mode, reward_per_point, point_limit] = subscriber.args.as_slice() else {
        return None;
    };
    if !super::subscriber_is_kind(subscriber, BuffActKind::TransferEnergyBuff)
        || !subscriber.owner_alive
        || change.kind != ExPointKind::Common
        || *transfer_mode <= 0
        || change.overflow <= 0
        || pool.entity(subscriber.owner_uid).is_none()
        || pool.entity(change.target_uid).is_none()
        || pool.source_is_attacker(subscriber.owner_uid)
            != pool.source_is_attacker(change.target_uid)
        || stores_overflow_moxie(managers, change.target_uid)
    {
        return Some(Vec::new());
    }
    let amount = change
        .overflow
        .min(*point_limit)
        .checked_mul(*reward_per_point)?;
    if amount <= 0 {
        return Some(Vec::new());
    }

    Some(vec![BuffActRuleOp::causing(RuleOp::Command(
        BattleCommand::Buff(BuffCommand::Grant(BuffGrant {
            origin: super::command_origin(subscriber)?,
            source_uid: if subscriber.source_uid != 0 {
                subscriber.source_uid
            } else {
                subscriber.owner_uid
            },
            target_uid: subscriber.owner_uid,
            buff_id: *buff_id,
            amount: Some(amount),
            occurrences: 1,
            child_uid_reservations: 0,
        })),
    ))])
}

fn stores_overflow_moxie(managers: &BattleManagers, owner_uid: i64) -> bool {
    managers
        .buff
        .active_features(&managers.hp)
        .into_iter()
        .any(|feature| {
            feature.owner_uid == owner_uid
                && super::is_kind(&feature, BuffActKind::ExPointOverflowBank)
        })
}

#[cfg(test)]
mod tests {
    use sonettobuf::{Fight, FightEntityInfo, FightTeam};

    use super::*;
    use crate::engine::{
        event::{kind::EventKind, payload::ExPointChangeEvent, subscription::SubscriptionKey},
        skill::rule::{DefinitionKey, RuleDomain},
    };

    #[test]
    fn parses_the_config_owned_transfer_rule() {
        assert_eq!(referenced_buff(&[31280113, 1, 20, 3]), Some(31280113));
        assert_eq!(referenced_buff(&[31280113, 1, 20]), None);
        assert_eq!(referenced_buff(&[0, 1, 20, 3]), None);
    }

    #[test]
    fn same_team_moxie_overflow_grants_the_configured_buff_amount() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![
                    FightEntityInfo {
                        uid: Some(10),
                        current_hp: Some(100),
                        team_type: Some(1),
                        ..Default::default()
                    },
                    FightEntityInfo {
                        uid: Some(11),
                        current_hp: Some(100),
                        team_type: Some(1),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            defender: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(-1),
                    current_hp: Some(100),
                    team_type: Some(2),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = BattleManagers::seeded(&fight);
        let pool = TargetPool::from_fight(&fight);
        let subscriber = BuffActSubscriber {
            owner_uid: 10,
            source_uid: 10,
            buff_uid: 20,
            buff_id: 31280118,
            team_type: 1,
            owner_alive: true,
            amount: 1,
            key: SubscriptionKey::new(
                EventKind::ExPointOverflow,
                DefinitionKey::new(1033, "TransferEnergyBuff"),
            ),
            act_type: "TransferEnergyBuff".into(),
            effect_time: 0,
            effect_condition: 0,
            args: vec![31280113, 1, 20, 3],
            raw: "1033#31280113#1#20#3".into(),
        };
        let event = |target_uid| {
            BattleEvent::ExPointOverflow(ExPointChangeEvent {
                origin: crate::engine::skill::rule::CommandOrigin {
                    domain: RuleDomain::Lifecycle,
                    key: DefinitionKey::new(0, "RoundRefill"),
                },
                source_uid: target_uid,
                target_uid,
                kind: ExPointKind::Common,
                before: 5,
                requested_delta: 2,
                applied_delta: 0,
                after: 5,
                overflow: 2,
            })
        };

        let ops = rule_ops(&managers, &pool, &subscriber, &event(11)).unwrap();
        assert!(matches!(
            ops.as_slice(),
            [BuffActRuleOp {
                op: RuleOp::Command(BattleCommand::Buff(BuffCommand::Grant(BuffGrant {
                    buff_id: 31280113,
                    amount: Some(40),
                    ..
                }))),
                ..
            }]
        ));
        assert!(
            rule_ops(&managers, &pool, &subscriber, &event(-1))
                .unwrap()
                .is_empty()
        );
    }
}
