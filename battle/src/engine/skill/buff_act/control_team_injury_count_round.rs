use crate::engine::{
    event::payload::BattleEvent,
    manager::{
        BattleManagers,
        injury::{InjuryCommand, InjuryRecord},
    },
    skill::{
        rule::output::{BattleCommand, RuleOp},
        subscriber::BuffActSubscriber,
        target::TargetPool,
    },
};

pub fn rule_ops(
    managers: &BattleManagers,
    pool: &TargetPool,
    subscriber: &BuffActSubscriber,
    event: &BattleEvent,
) -> Option<Vec<RuleOp>> {
    let BattleEvent::HpLost {
        source_uid,
        target_uid,
        ..
    } = event
    else {
        return None;
    };
    if !super::subscriber_is_kind(
        subscriber,
        super::registry::BuffActKind::ControlTeamInjuryCountRound,
    ) || !super::is_primary_team_subscriber(
        managers,
        subscriber,
        super::registry::BuffActKind::ControlTeamInjuryCountRound,
    ) || pool.team_type(*target_uid) != Some(subscriber.team_type)
    {
        return Some(Vec::new());
    }
    Some(vec![RuleOp::Command(BattleCommand::Injury(
        InjuryCommand::RecordAction(InjuryRecord {
            origin: super::command_origin(subscriber)?,
            source_uid: *source_uid,
            team_type: subscriber.team_type,
            injured_targets: vec![*target_uid],
            counter_owner_uid: subscriber.owner_uid,
        }),
    ))])
}

pub fn scoped_rule_ops(
    managers: &BattleManagers,
    pool: &TargetPool,
    subscriber: &BuffActSubscriber,
    event: &BattleEvent,
) -> Option<Vec<super::BuffActRuleOp>> {
    rule_ops(managers, pool, subscriber, event)
        .map(|ops| ops.into_iter().map(super::BuffActRuleOp::event).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{
        event::{kind::EventKind, subscription::SubscriptionKey},
        skill::rule::{CommandOrigin, DefinitionKey, RuleDomain},
    };
    use sonettobuf::{Fight, FightEntityInfo, FightTeam};

    #[test]
    fn each_committed_ally_hp_loss_records_one_team_injury() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(10),
                    team_type: Some(1),
                    buffs: vec![sonettobuf::BuffInfo {
                        uid: Some(20),
                        buff_id: Some(308801311),
                        from_uid: Some(10),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let pool = TargetPool::from_fight(&fight);
        let managers = BattleManagers::seeded(&fight);
        let subscriber = BuffActSubscriber {
            owner_uid: 10,
            source_uid: 10,
            buff_uid: 20,
            buff_id: 308801311,
            team_type: 1,
            owner_alive: true,
            amount: 0,
            key: SubscriptionKey::new(
                EventKind::HpLost,
                DefinitionKey::new(760, "ControlTeamInjuryCountRound"),
            ),
            act_type: "ControlTeamInjuryCountRound".into(),
            effect_time: 0,
            effect_condition: 0,
            args: Vec::new(),
            raw: "760".into(),
        };
        let event = BattleEvent::HpLost {
            origin: CommandOrigin {
                domain: RuleDomain::Skill,
                key: DefinitionKey::new(1, "TestHpLoss"),
            },
            source_uid: 10,
            skill_id: 1,
            target_uid: 10,
            amount: 100,
            buff_uid: None,
        };

        assert!(matches!(
            rule_ops(&managers, &pool, &subscriber, &event)
                .unwrap()
                .as_slice(),
            [RuleOp::Command(BattleCommand::Injury(InjuryCommand::RecordAction(
                InjuryRecord { injured_targets, counter_owner_uid: 10, .. }
            )))] if injured_targets == &[10]
        ));
    }
}
