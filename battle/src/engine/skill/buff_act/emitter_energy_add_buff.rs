use crate::engine::{
    event::payload::BattleEvent,
    manager::{
        BattleManagers,
        buff::{BuffCommand, BuffGrantChild},
    },
    mechanic::impromptu,
    skill::{
        buff_act::{BuffActRuleOp, registry::BuffActKind},
        rule::output::{BattleCommand, RuleOp},
        subscriber::BuffActSubscriber,
    },
};

pub fn supports(args: &[i32]) -> bool {
    matches!(args, [required_energy, buff_id] if *required_energy > 0 && *buff_id > 0)
}

pub fn rule_ops(
    managers: &BattleManagers,
    subscriber: &BuffActSubscriber,
    event: &BattleEvent,
) -> Option<Vec<BuffActRuleOp>> {
    if !super::subscriber_is_kind(subscriber, BuffActKind::EmitterEnergyAddBuff) {
        return Some(Vec::new());
    }
    let BattleEvent::PlayerActionsResolved { team, emitter_uid } = event else {
        return Some(Vec::new());
    };
    if *team != subscriber.team_type || *emitter_uid != subscriber.owner_uid {
        return Some(Vec::new());
    }
    let [required_energy, buff_id, ..] = subscriber.args.as_slice() else {
        return None;
    };
    let energy = managers
        .gauge
        .get(impromptu::inspiration_key(*emitter_uid))
        .map(|state| state.current)
        .unwrap_or_default();
    if energy < *required_energy || managers.buff.has_buff_id(*emitter_uid, *buff_id) {
        return Some(Vec::new());
    }

    Some(vec![BuffActRuleOp::independent_event(RuleOp::Command(
        BattleCommand::Buff(BuffCommand::GrantInternalChild(BuffGrantChild {
            origin: super::command_origin(subscriber)?,
            source_uid: *emitter_uid,
            target_uid: *emitter_uid,
            buff_id: *buff_id,
            amount: Some(0),
            params: None,
            act_info: None,
        })),
    ))])
}

#[cfg(test)]
mod tests {
    use sonettobuf::{BuffInfo, Fight, FightEntityInfo, FightTeam};

    use super::*;
    use crate::engine::{
        event::kind::EventKind,
        manager::{BattleManagers, emitter, gauge::GaugeCommand},
        skill::{buff_act, rule::output::BattleCommand},
    };

    #[test]
    fn inspiration_thresholds_activate_internal_attack_buffs() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(emitter::UID),
                    team_type: Some(1),
                    current_hp: Some(1),
                    buffs: vec![BuffInfo {
                        uid: Some(30),
                        buff_id: Some(31080151),
                        from_uid: Some(emitter::UID),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut managers = BattleManagers::seeded(&fight);
        let subscriber = crate::engine::skill::subscriber::for_active_buffs(
            &managers,
            EventKind::PlayerActionsResolved,
        )
        .into_iter()
        .find(|subscriber| subscriber.args.first() == Some(&3))
        .unwrap();
        managers
            .execute_gauge(GaugeCommand::new(
                buff_act::command_origin(&subscriber).unwrap(),
                impromptu::inspiration_key(emitter::UID),
                crate::engine::manager::gauge::GaugeOperation::Enable { max: None },
            ))
            .unwrap();
        managers
            .execute_gauge(GaugeCommand::new(
                buff_act::command_origin(&subscriber).unwrap(),
                impromptu::inspiration_key(emitter::UID),
                crate::engine::manager::gauge::GaugeOperation::ChangeValue { delta: 3 },
            ))
            .unwrap();

        let event = BattleEvent::PlayerActionsResolved {
            team: 1,
            emitter_uid: emitter::UID,
        };
        let ops = rule_ops(&managers, &subscriber, &event).unwrap();
        let [op] = ops.as_slice() else {
            panic!("expected one internal child grant")
        };
        let RuleOp::Command(BattleCommand::Buff(command)) = op.op.clone() else {
            panic!("expected a buff command")
        };
        let changes = managers.execute_buff(command).unwrap();

        assert!(managers.buff.has_buff_id(emitter::UID, 31080152));
        assert!(!changes.is_wire_visible());
        assert!(changes.events().is_empty());
    }
}
