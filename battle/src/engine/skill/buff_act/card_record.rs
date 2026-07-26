use crate::engine::{
    event::payload::BattleEvent,
    manager::{
        BattleManagers,
        buff::{BuffCommand, BuffSetState},
    },
    skill::effect::SkillEffectCatalog,
    skill::{
        rule::output::{BattleCommand, RuleOp},
        subscriber::BuffActSubscriber,
    },
};

pub(crate) fn recorded_skill_ids(managers: &BattleManagers, owner_uid: i64) -> Vec<i32> {
    let Some((act_id, params)) = managers.buff.buff_act_state(
        owner_uid,
        super::registry::BuffActKind::AddCardRecordByRound,
    ) else {
        return Vec::new();
    };
    parse_recorded_skill_ids(Some(params), act_id)
}

fn parse_recorded_skill_ids(params: Option<&str>, act_id: i32) -> Vec<i32> {
    let mut values = params.unwrap_or_default().split('#');
    if values.next().and_then(|value| value.parse::<i32>().ok()) != Some(act_id) {
        return Vec::new();
    }
    values
        .filter_map(|value| value.parse::<i32>().ok())
        .filter(|skill_id| *skill_id > 0)
        .collect()
}

pub fn rule_ops(
    managers: &BattleManagers,
    catalog: &SkillEffectCatalog,
    subscriber: &BuffActSubscriber,
    event: &BattleEvent,
) -> Option<Vec<RuleOp>> {
    let BattleEvent::ActionQueueCommitted { .. } = event else {
        return None;
    };
    let limit = subscriber.args.first().copied().unwrap_or_default().max(0) as usize;
    let skill_ids = managers
        .card
        .played()
        .iter()
        .filter_map(|played| played.card.skill_id)
        .filter(|skill_id| !catalog.is_big_skill(*skill_id))
        .take(limit);
    let params = std::iter::once(subscriber.key.definition.opcode.to_string())
        .chain(skill_ids.map(|skill_id| skill_id.to_string()))
        .collect::<Vec<_>>()
        .join("#");
    Some(vec![RuleOp::Command(BattleCommand::Buff(
        BuffCommand::SetStateSnapshot(BuffSetState {
            ex_info: None,
            origin: super::command_origin(subscriber)?,
            target_uid: subscriber.owner_uid,
            buff_uid: subscriber.buff_uid,
            params: Some(params),
            act_info: None,
        }),
    ))])
}

#[cfg(test)]
mod tests {
    use super::parse_recorded_skill_ids;

    #[test]
    fn parses_only_the_exact_record_state() {
        assert_eq!(
            parse_recorded_skill_ids(Some("929#10#20"), 929),
            vec![10, 20]
        );
        assert!(parse_recorded_skill_ids(Some("923#10#20"), 929).is_empty());
        assert!(parse_recorded_skill_ids(None, 929).is_empty());
    }
}
