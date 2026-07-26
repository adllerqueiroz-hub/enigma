use crate::engine::{
    event::{kind::EventKind, payload::BattleEvent},
    skill::{
        action::{SkillInvocation, SkillRequest},
        buff_act::registry::BuffActKind,
        rule::output::RuleOp,
        subscriber::BuffActSubscriber,
    },
};

pub fn rule_ops(subscriber: &BuffActSubscriber, event: &BattleEvent) -> Option<Vec<RuleOp>> {
    if !super::subscriber_is_kind(subscriber, BuffActKind::PaperCircleContinueChannel) {
        return None;
    }
    if !matches!(event, BattleEvent::Kind(EventKind::RoundEnd)) {
        return Some(Vec::new());
    }
    let skill_id = referenced_skill(&subscriber.raw)?;
    Some(vec![RuleOp::Skill(SkillInvocation::from(SkillRequest {
        source_uid: subscriber.owner_uid,
        skill_id,
    }))])
}

pub fn referenced_skill(raw: &str) -> Option<i32> {
    let fields = raw.split('#').collect::<Vec<_>>();
    let [act, skill, value0, value1, levels, field_ids] = fields.as_slice() else {
        return None;
    };
    if act.parse::<i32>().ok()? != 862
        || value0.parse::<i32>().is_err()
        || value1.parse::<i32>().is_err()
    {
        return None;
    }
    let parse_group = |raw: &str| {
        raw.split(',')
            .map(str::trim)
            .map(str::parse::<i32>)
            .collect::<Result<Vec<_>, _>>()
            .ok()
    };
    let levels = parse_group(levels)?;
    let field_ids = parse_group(field_ids)?;
    let skill_id = skill.parse::<i32>().ok()?;
    (skill_id > 0 && !levels.is_empty() && levels.len() == field_ids.len()).then_some(skill_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{event::subscription::SubscriptionKey, skill::rule::DefinitionKey};

    #[test]
    fn round_end_casts_the_configured_continuation_skill() {
        let subscriber = BuffActSubscriber {
            owner_uid: 10,
            source_uid: 10,
            buff_uid: 20,
            buff_id: 30,
            team_type: 1,
            owner_alive: true,
            amount: 1,
            key: SubscriptionKey::new(
                EventKind::RoundEnd,
                DefinitionKey::new(862, "PaperCircleContinueChannel"),
            ),
            act_type: "PaperCircleContinueChannel".to_owned(),
            effect_time: 302,
            effect_condition: 0,
            args: vec![31050152, 3, 210, 2, 3, 4, 31050181, 31050182, 31050183],
            raw: "862#31050152#3#210#2,3,4#31050181,31050182,31050183".to_owned(),
        };

        assert!(matches!(
            rule_ops(&subscriber, &BattleEvent::Kind(EventKind::RoundEnd)).as_deref(),
            Some([RuleOp::Skill(SkillInvocation {
                plan: SkillRequest {
                    source_uid: 10,
                    skill_id: 31050152,
                },
                ..
            })])
        ));
    }
}
