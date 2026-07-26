use sonettobuf::effect_type_enum::EffectType;

use crate::engine::{
    event::{kind::EventKind, payload::BattleEvent},
    manager::{
        BattleManagers,
        buff::{BuffCommand, BuffGrant, BuffRemove, BuffRemoveSelector, BuffSetState},
        ex_point::{ExPointChange, ExPointCommand},
    },
    skill::{
        action::{SkillInvocation, SkillRequest},
        buff_act::registry::BuffActKind,
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
    if !super::subscriber_is_kind(subscriber, BuffActKind::DuduBoneContinueChannel) {
        return None;
    }
    if !matches!(event, BattleEvent::Kind(EventKind::SmallRoundEnd)) {
        return Some(Vec::new());
    }
    let [
        bane_buff_id,
        _compound_cap,
        first_stacks,
        ending_skill,
        _target_code,
    ] = subscriber.args.as_slice()
    else {
        return None;
    };
    if *bane_buff_id <= 0 || *first_stacks <= 0 || *ending_skill <= 0 {
        return None;
    }

    let origin = super::command_origin(subscriber)?;
    let completed_rounds = managers
        .buff
        .snapshot(subscriber.owner_uid, subscriber.buff_uid)?
        .ex_info
        .unwrap_or_default()
        .max(0);
    let available = managers.ex_point.get(subscriber.owner_uid).max(0);
    let requested = if completed_rounds == 0 {
        0
    } else {
        2_i32.saturating_add(completed_rounds.saturating_mul(2))
    };
    let spent = available.min(requested);
    let ending = requested > available;
    let stacks = if completed_rounds == 0 {
        *first_stacks
    } else {
        spent / 2
    };

    let mut ops = Vec::new();
    if spent > 0 {
        ops.push(RuleOp::Command(BattleCommand::ExPoint(
            ExPointCommand::Change(ExPointChange {
                origin,
                source_uid: subscriber.owner_uid,
                target_uid: subscriber.owner_uid,
                delta: -spent,
                config_effect: 0,
                effect_type: EffectType::Expointchange as i32,
            }),
        )));
    }
    if stacks > 0 {
        ops.extend(
            pool.enemies(subscriber.owner_uid, false)
                .iter()
                .filter(|enemy| managers.hp.current(enemy.uid) > 0)
                .map(|enemy| {
                    RuleOp::Command(BattleCommand::Buff(BuffCommand::Grant(BuffGrant {
                        origin,
                        source_uid: subscriber.owner_uid,
                        target_uid: enemy.uid,
                        buff_id: *bane_buff_id,
                        amount: Some(stacks),
                        occurrences: 1,
                        child_uid_reservations: 0,
                    })))
                }),
        );
    }
    if ending {
        let invocation: SkillInvocation = SkillRequest {
            source_uid: subscriber.owner_uid,
            skill_id: *ending_skill,
        }
        .into();
        ops.push(RuleOp::Skill(invocation));
        ops.push(RuleOp::Command(BattleCommand::Buff(BuffCommand::Remove(
            BuffRemove {
                origin,
                target_uid: subscriber.owner_uid,
                selector: BuffRemoveSelector::Uid(subscriber.buff_uid),
            },
        ))));
    } else {
        ops.push(RuleOp::Command(BattleCommand::Buff(BuffCommand::SetState(
            BuffSetState {
                origin,
                target_uid: subscriber.owner_uid,
                buff_uid: subscriber.buff_uid,
                ex_info: Some(completed_rounds.saturating_add(1)),
                params: None,
                act_info: None,
            },
        ))));
    }
    Some(ops)
}

pub fn compound_offset(managers: &BattleManagers, source_uid: i64) -> i32 {
    managers
        .buff
        .active_features(&managers.hp)
        .into_iter()
        .filter(|feature| feature.owner_uid == source_uid)
        .filter(|feature| super::is_kind(feature, BuffActKind::DuduBoneContinueChannel))
        .filter_map(|feature| {
            let [act_id, _bane_buff_id, cap, ..] = feature.values.as_slice() else {
                return None;
            };
            let _ = act_id;
            Some(
                managers
                    .buff
                    .snapshot(feature.owner_uid, feature.buff_uid)?
                    .ex_info
                    .unwrap_or_default()
                    .min(*cap)
                    .max(0),
            )
        })
        .max()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use sonettobuf::{Fight, FightEntityInfo, FightTeam};

    use super::*;
    use crate::engine::{
        event::bus::EventBus,
        runtime::executor::execute_rule_op,
        skill::{buff_act, subscriber},
    };

    #[test]
    fn channel_keeps_elapsed_rounds_in_buff_ex_info_and_ends_when_moxie_is_short() {
        crate::test_support::init_config();
        let entity = |uid, team_type, ex_point| FightEntityInfo {
            uid: Some(uid),
            team_type: Some(team_type),
            current_hp: Some(1_000),
            ex_point: Some(ex_point),
            ..Default::default()
        };
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![entity(10, 1, 5)],
                ..Default::default()
            }),
            defender: Some(FightTeam {
                entitys: vec![entity(-1, 2, 0)],
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut managers = BattleManagers::seeded(&fight);
        let pool = TargetPool::from_fight(&fight);
        managers.buff.add(&managers.hp, 10, 10, 31040014, 0);
        let subscriber = subscriber::for_active_buffs(&managers, EventKind::SmallRoundEnd)
            .into_iter()
            .find(|subscriber| {
                buff_act::subscriber_is_kind(subscriber, BuffActKind::DuduBoneContinueChannel)
            })
            .unwrap();
        let event = BattleEvent::Kind(EventKind::SmallRoundEnd);
        let mut bus = EventBus::default();

        let first = rule_ops(&managers, &pool, &subscriber, &event).unwrap();
        assert_eq!(first.len(), 2);
        for op in first {
            execute_rule_op(&mut managers, &mut bus, op).unwrap();
        }
        assert_eq!(managers.ex_point.get(10), 5);
        assert_eq!(
            managers
                .buff
                .snapshot(10, subscriber.buff_uid)
                .unwrap()
                .ex_info,
            Some(1)
        );
        assert_eq!(managers.buff.buff_id_amount(-1, 31040013), 1);

        let second = rule_ops(&managers, &pool, &subscriber, &event).unwrap();
        assert_eq!(second.len(), 3);
        for op in second {
            execute_rule_op(&mut managers, &mut bus, op).unwrap();
        }
        assert_eq!(managers.ex_point.get(10), 1);
        assert_eq!(
            managers
                .buff
                .snapshot(10, subscriber.buff_uid)
                .unwrap()
                .ex_info,
            Some(2)
        );
        assert_eq!(managers.buff.buff_id_amount(-1, 31040013), 3);
        assert_eq!(compound_offset(&managers, 10), 2);

        let ending = rule_ops(&managers, &pool, &subscriber, &event).unwrap();
        assert!(matches!(
            ending.as_slice(),
            [
                RuleOp::Command(BattleCommand::ExPoint(_)),
                RuleOp::Skill(SkillInvocation {
                    plan: SkillRequest {
                        skill_id: 310401311,
                        ..
                    },
                    ..
                }),
                RuleOp::Command(BattleCommand::Buff(BuffCommand::Remove(_)))
            ]
        ));
    }
}
