use crate::engine::{
    event::payload::BattleEvent,
    manager::{
        BattleManagers,
        buff::{BuffCommand, BuffSetState},
        card::{CardAddPrecast, CardCommand, temp::runtime_precast_card},
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
    println!("DEBUG BUTTERFLY: event={:?}", event);
    if !matches!(
        event,
        BattleEvent::Kind(crate::engine::event::kind::EventKind::RoundStartCard)
    ) {
        return None;
    }

    let count = subscriber.args.first().copied().unwrap_or(1);
    println!("DEBUG BUTTERFLY: count={}", count);
    if count <= 0 {
        return Some(Vec::new());
    }

    let recorded = super::card_record::recorded_skill_ids(managers, subscriber.owner_uid);
    println!("DEBUG BUTTERFLY: recorded skill_ids={:?}", recorded);
    
    let Some(skill_id) = recorded.last().copied() else {
        println!("DEBUG BUTTERFLY: nenhuma skill gravada encontrada no AddCardRecordByRound!");
        return Some(Vec::new());
    };
    println!("DEBUG BUTTERFLY: target skill_id={}", skill_id);

    let enchant_id = subscriber.args.get(1).copied()?;
    let owner_uid = pool
        .allies(subscriber.owner_uid)
        .iter()
        .find(|entity| {
            entity.skill_group1.contains(&skill_id)
                || entity.skill_group2.contains(&skill_id)
                || crate::engine::mechanic::card::CardMechanic
                    .is_ultimate_skill(managers, skill_id, entity)
        })?
        .uid;

    let origin = super::command_origin(subscriber)?;
    let mut card = runtime_precast_card(managers, owner_uid, skill_id);
    card.enchants.push(sonettobuf::CardEnchant {
        enchant_id: Some(enchant_id),
        duration: Some(1),
        ex_info: Vec::new(),
    });

    let mut ops = (0..count)
        .map(|_| {
            RuleOp::Command(BattleCommand::Card(CardCommand::AddPrecast(
                CardAddPrecast {
                    origin,
                    card: card.clone(),
                },
            )))
        })
        .collect::<Vec<_>>();

    ops.push(RuleOp::Command(BattleCommand::Buff(BuffCommand::SetState(
        BuffSetState {
            origin,
            target_uid: subscriber.owner_uid,
            buff_uid: subscriber.buff_uid,
            ex_info: None,
            params: None,
            act_info: Some(vec![sonettobuf::BuffActInfo {
                act_id: Some(subscriber.key.definition.opcode),
                param: Vec::new(),
                str_param: Some(String::new()),
            }]),
        },
    ))));

    Some(ops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{
        event::{kind::EventKind, subscription::SubscriptionKey},
        skill::rule::DefinitionKey,
    };

    #[test]
    fn empty_record_is_a_valid_round_start_noop() {
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
                DefinitionKey::new(1104, "ButterflyRecordSkill"),
            ),
            act_type: "ButterflyRecordSkill".to_owned(),
            effect_time: 105,
            effect_condition: 0,
            args: vec![1, 218, 1],
            raw: "1104#1#218#1".to_owned(),
        };

        assert!(
            rule_ops(
                &BattleManagers::default(),
                &TargetPool::default(),
                &subscriber,
                &BattleEvent::Kind(EventKind::RoundStartCard),
            )
            .is_some_and(|ops| ops.is_empty())
        );
    }
}