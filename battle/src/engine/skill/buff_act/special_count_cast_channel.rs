use crate::engine::{
    event::{kind::EventKind, payload::BattleEvent},
    skill::{
        action::{SkillExecutionMode, SkillInvocation, SkillRequest},
        buff_act::registry::BuffActKind,
        condition::extra::skill_kind_from_is_extra,
        effect::SkillEffectCatalog,
        rule::output::RuleOp,
        subscriber::BuffActSubscriber,
    },
};

pub fn rule_ops(
    subscriber: &BuffActSubscriber,
    event: &BattleEvent,
    catalog: &SkillEffectCatalog,
) -> Option<Vec<RuleOp>> {
    if !super::subscriber_is_kind(subscriber, BuffActKind::SpecialCountCastChannel) {
        return None;
    }
    if !matches!(
        event,
        BattleEvent::Kind(EventKind::RoundEndEntitySettlement)
    ) {
        return Some(Vec::new());
    }
    let skill_id = subscriber.args.first().copied().filter(|id| *id > 0)?;
    let mut invocation: SkillInvocation = SkillRequest {
        source_uid: subscriber.owner_uid,
        skill_id,
    }
    .into();
    invocation.mode = SkillExecutionMode::Active;
    invocation.extra_skill_kind = skill_kind_from_is_extra(catalog.extra_kind(skill_id));
    Some(vec![RuleOp::Skill(invocation)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{event::subscription::SubscriptionKey, skill::rule::DefinitionKey};

    #[test]
    fn settlement_casts_the_configured_extra_action() {
        crate::test_support::init_config();
        let catalog = SkillEffectCatalog::from_game_db(config::configs::get());
        let subscriber = BuffActSubscriber {
            owner_uid: 10,
            source_uid: 10,
            buff_uid: 20,
            buff_id: 31070131,
            team_type: 1,
            owner_alive: true,
            amount: 1,
            key: SubscriptionKey::new(
                EventKind::RoundEndEntitySettlement,
                DefinitionKey::new(1002, "SpecialCountCastChannel"),
            ),
            act_type: "SpecialCountCastChannel".to_owned(),
            effect_time: 303,
            effect_condition: 0,
            args: vec![31070151, 1, 202, 1, 10],
            raw: "1002#31070151#1#202#1#10".to_owned(),
        };

        assert!(matches!(
            rule_ops(
                &subscriber,
                &BattleEvent::Kind(EventKind::RoundEndEntitySettlement),
                &catalog,
            )
            .as_deref(),
            Some([RuleOp::Skill(SkillInvocation {
                plan: SkillRequest {
                    source_uid: 10,
                    skill_id: 31070151,
                },
                mode: SkillExecutionMode::Active,
                extra_skill_kind: Some(
                    crate::engine::skill::condition::extra::ExtraSkillKind::ExtraAction
                ),
                ..
            })])
        ));
    }
}
