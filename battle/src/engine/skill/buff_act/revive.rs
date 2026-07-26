use crate::engine::{
    entity::attr::AttrId,
    event::payload::BattleEvent,
    manager::{BattleManagers, buff::BuffStatus, revive::ReviveCommand},
    skill::{
        rule::output::{BattleCommand, RuleOp},
        subscriber::BuffActSubscriber,
    },
};

pub fn supports(args: &[i32]) -> bool {
    matches!(args, [0 | 1, raw_attr, permille]
        if AttrId::from_raw(*raw_attr).is_some() && *permille > 0)
}

pub fn supports_dying_heal(args: &[i32]) -> bool {
    matches!(args, [permille, statuses @ ..]
        if *permille > 0
            && !statuses.is_empty()
            && statuses.iter().all(|status| BuffStatus::from_id(*status).is_bad()))
}

pub fn rule_ops(
    managers: &BattleManagers,
    subscriber: &BuffActSubscriber,
    event: &BattleEvent,
) -> Option<Vec<RuleOp>> {
    let BattleEvent::EntityDied(death) = event else {
        return None;
    };
    if death.target_uid != subscriber.owner_uid || subscriber.owner_alive {
        return Some(Vec::new());
    }
    let source_uid = if subscriber.source_uid != 0 {
        subscriber.source_uid
    } else {
        subscriber.owner_uid
    };
    let (amount, dispel_statuses) = if matches!(
        super::subscriber_kind(subscriber),
        Some(super::registry::BuffActKind::Revive | super::registry::BuffActKind::Cure)
    ) {
        let [mode, raw_attr, permille] = subscriber.args.as_slice() else {
            return None;
        };
        if !supports(&subscriber.args) {
            return None;
        }
        let basis_uid = if *mode == 0 {
            source_uid
        } else {
            subscriber.owner_uid
        };
        (
            managers
                .origin_attribute(basis_uid, AttrId::from_raw(*raw_attr)?)
                .max(0)
                .saturating_mul(*permille)
                / 1000,
            Vec::new(),
        )
    } else if super::subscriber_is_kind(
        subscriber,
        super::registry::BuffActKind::DyingHealDisperse1,
    ) {
        let [permille, statuses @ ..] = subscriber.args.as_slice() else {
            return None;
        };
        if !supports_dying_heal(&subscriber.args) {
            return None;
        }
        (
            managers.hp.max(subscriber.owner_uid).max(0) * *permille / 1000,
            statuses.iter().copied().map(BuffStatus::from_id).collect(),
        )
    } else {
        return None;
    };
    let origin = super::command_origin(subscriber)?;
    Some(if amount > 0 {
        vec![RuleOp::Command(BattleCommand::Revive(ReviveCommand {
            origin,
            source_uid,
            target_uid: subscriber.owner_uid,
            amount,
            buff_uid: subscriber.buff_uid,
            dispel_statuses,
        }))]
    } else {
        Vec::new()
    })
}

#[cfg(test)]
mod tests {
    use sonettobuf::{BuffInfo, Fight, FightEntityInfo, FightTeam, HeroAttribute};

    use super::*;
    use crate::engine::{
        event::{kind::EventKind, payload::EntityDiedEvent, subscription::SubscriptionKey},
        skill::rule::DefinitionKey,
    };

    #[test]
    fn source_scaled_revive_emits_one_atomic_transaction() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(10),
                    current_hp: Some(0),
                    attr: Some(HeroAttribute {
                        hp: Some(20_000),
                        ..Default::default()
                    }),
                    buffs: vec![BuffInfo {
                        uid: Some(20),
                        buff_id: Some(31250181),
                        from_uid: Some(10),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = BattleManagers::seeded(&fight);
        let subscriber = BuffActSubscriber {
            owner_uid: 10,
            source_uid: 10,
            buff_uid: 20,
            buff_id: 31250181,
            team_type: 1,
            owner_alive: false,
            amount: 1,
            key: SubscriptionKey::new(EventKind::EntityDied, DefinitionKey::new(1043, "Revive")),
            act_type: "Revive".to_owned(),
            effect_time: 12,
            effect_condition: 0,
            args: vec![0, AttrId::Hp as i32, 300],
            raw: "1043#0#101#300".to_owned(),
        };

        assert!(matches!(
            rule_ops(
                &managers,
                &subscriber,
                &BattleEvent::EntityDied(EntityDiedEvent {
                    source_uid: -1,
                    target_uid: 10,
                }),
            )
            .unwrap()
            .as_slice(),
            [RuleOp::Command(BattleCommand::Revive(ReviveCommand {
                amount: 6_000,
                ..
            }))]
        ));
    }

    #[test]
    fn lethal_cure_recovers_configured_max_hp_and_consumes_its_buff() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(10),
                    current_hp: Some(0),
                    attr: Some(HeroAttribute {
                        hp: Some(20_000),
                        ..Default::default()
                    }),
                    buffs: vec![BuffInfo {
                        uid: Some(20),
                        buff_id: Some(433021),
                        from_uid: Some(10),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = BattleManagers::seeded(&fight);
        let subscriber = BuffActSubscriber {
            owner_uid: 10,
            source_uid: 10,
            buff_uid: 20,
            buff_id: 433021,
            team_type: 1,
            owner_alive: false,
            amount: 1,
            key: SubscriptionKey::new(EventKind::EntityDied, DefinitionKey::new(512, "Cure")),
            act_type: "Cure".to_owned(),
            effect_time: 12,
            effect_condition: 0,
            args: vec![1, AttrId::Hp as i32, 330],
            raw: "512#1#101#330".to_owned(),
        };

        assert!(matches!(
            rule_ops(
                &managers,
                &subscriber,
                &BattleEvent::EntityDied(EntityDiedEvent {
                    source_uid: -1,
                    target_uid: 10,
                }),
            )
            .unwrap()
            .as_slice(),
            [RuleOp::Command(BattleCommand::Revive(ReviveCommand {
                amount: 6_600,
                buff_uid: 20,
                ..
            }))]
        ));
    }
}
