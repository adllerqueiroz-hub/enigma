use crate::engine::{
    event::payload::BattleEvent,
    manager::{
        BattleManagers,
        buff::{BuffCommand, BuffGrant, BuffRemove, BuffRemoveSelector},
    },
    skill::{
        buff_act::{command_origin, registry::BuffActKind, subscriber_is_kind},
        rule::output::{BattleCommand, RuleOp},
        subscriber::BuffActSubscriber,
        target::TargetPool,
    },
};

pub fn rule_ops(
    managers: &BattleManagers,
    pool: &TargetPool,
    determinism: &mut crate::engine::runtime::determinism::RoundDeterminism,
    subscriber: &BuffActSubscriber,
    event: &BattleEvent,
) -> Option<Vec<RuleOp>> {
    if !subscriber_is_kind(subscriber, BuffActKind::AddBuffBoth) {
        return None;
    }
    let BattleEvent::AllyAction(action) = event else {
        return None;
    };
    let [enemy_buff_id, ally_target_code, ally_buff_id] = subscriber.args.as_slice() else {
        return None;
    };
    if *enemy_buff_id <= 0 || *ally_target_code <= 0 || *ally_buff_id <= 0 {
        return None;
    }

    let origin = command_origin(subscriber)?;
    let source_uid = if subscriber.source_uid == 0 {
        subscriber.owner_uid
    } else {
        subscriber.source_uid
    };
    let ally_targets =
        crate::engine::skill::target::TargetResolver::resolve_with_managers_and_context(
            &crate::engine::skill::target::TargetRequest {
                code: *ally_target_code,
                raw: Vec::new(),
            },
            action.skill_id,
            subscriber.owner_uid,
            pool,
            determinism,
            Some(managers),
            Default::default(),
        );
    let grants = action
        .target_uids
        .iter()
        .copied()
        .filter(|uid| {
            pool.enemies(subscriber.owner_uid, true)
                .iter()
                .any(|enemy| enemy.uid == *uid)
        })
        .map(|uid| (uid, *enemy_buff_id))
        .chain(ally_targets.into_iter().map(|uid| (uid, *ally_buff_id)))
        .map(|(target_uid, buff_id)| {
            RuleOp::Command(BattleCommand::Buff(BuffCommand::Grant(BuffGrant {
                origin,
                source_uid,
                target_uid,
                buff_id,
                amount: Some(1),
                occurrences: 1,
                child_uid_reservations: 0,
            })))
        });

    Some(
        grants
            .chain(std::iter::once(RuleOp::Command(BattleCommand::Buff(
                BuffCommand::Remove(BuffRemove {
                    origin,
                    target_uid: subscriber.owner_uid,
                    selector: BuffRemoveSelector::Uid(subscriber.buff_uid),
                }),
            ))))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{
        event::{kind::EventKind, subscription::SubscriptionKey},
        skill::{action::ActionEvent, rule::DefinitionKey},
    };
    use sonettobuf::{Fight, FightEntityInfo, FightTeam};

    #[test]
    fn grants_configured_buffs_to_enemy_and_allies_then_removes_carrier() {
        let entity = |uid| FightEntityInfo {
            uid: Some(uid),
            current_hp: Some(100),
            ..Default::default()
        };
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![entity(10), entity(11)],
                ..Default::default()
            }),
            defender: Some(FightTeam {
                entitys: vec![entity(-1), entity(-2)],
                ..Default::default()
            }),
            ..Default::default()
        };
        let pool = TargetPool::from_fight(&fight);
        let managers = BattleManagers::seeded(&fight);
        let subscriber = BuffActSubscriber {
            owner_uid: 10,
            source_uid: 10,
            buff_uid: 22,
            buff_id: 30091120,
            team_type: 1,
            owner_alive: true,
            amount: 1,
            key: SubscriptionKey::new(EventKind::SkillCast, DefinitionKey::new(850, "AddBuffBoth")),
            act_type: "AddBuffBoth".to_owned(),
            effect_time: 208,
            effect_condition: 0,
            args: vec![300901412, 101, 30091111],
            raw: "850#300901412#101#30091111".to_owned(),
        };
        let event = BattleEvent::AllyAction(ActionEvent {
            source_uid: 10,
            skill_id: 1,
            target_uid: -1,
            target_uids: vec![-1, -2],
            skill_slot: 1,
            is_attack: true,
            rank: 1,
            skill_type: 1,
            effect_tag: 0,
            additional_moxie: 0,
            extra_skill_kind: 0,
            assassinate: false,
            ..Default::default()
        });

        let ops = rule_ops(
            &managers,
            &pool,
            &mut Default::default(),
            &subscriber,
            &event,
        )
        .unwrap();
        let grants = ops
            .iter()
            .filter_map(|op| match op {
                RuleOp::Command(BattleCommand::Buff(BuffCommand::Grant(grant))) => {
                    Some((grant.target_uid, grant.buff_id))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(
            grants,
            vec![
                (-1, 300901412),
                (-2, 300901412),
                (10, 30091111),
                (11, 30091111)
            ]
        );
        assert!(matches!(
            ops.last(),
            Some(RuleOp::Command(BattleCommand::Buff(BuffCommand::Remove(
                BuffRemove {
                    target_uid: 10,
                    selector: BuffRemoveSelector::Uid(22),
                    ..
                }
            ))))
        ));
    }
}
