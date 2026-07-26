use crate::engine::skill::{
    action::SkillRequest,
    buff_act::registry::{self, BuffActKind},
    subscriber::BuffActSubscriber,
};

pub fn linked(subscriber: &BuffActSubscriber) -> Option<SkillRequest> {
    linked_for(
        subscriber.owner_uid,
        subscriber.key.definition.opcode,
        &subscriber.act_type,
        &subscriber.args,
    )
}

pub fn linked_for(
    owner_uid: i64,
    opcode: i32,
    type_name: &str,
    args: &[i32],
) -> Option<SkillRequest> {
    let skill_id = match registry::kind(opcode, type_name)? {
        BuffActKind::UseSkillToEnemy
        | BuffActKind::ConsumeBuffContinueChannel
        | BuffActKind::ConsumeBuffAddBuffContinueChannel => args.first(),
        BuffActKind::MonitorContinueChannel => args.get(1),
        _ => None,
    }
    .copied()
    .filter(|id| *id > 0)?;
    Some(SkillRequest {
        source_uid: owner_uid,
        skill_id,
    })
}

#[cfg(test)]
mod tests {
    use crate::engine::event::{kind::EventKind, subscription::SubscriptionKey};

    use super::*;

    fn subscriber(act_type: &'static str, act_id: i32, args: Vec<i32>) -> BuffActSubscriber {
        BuffActSubscriber {
            owner_uid: 10,
            source_uid: 10,
            buff_uid: 20,
            buff_id: 30,
            team_type: 1,
            owner_alive: true,
            amount: 0,
            key: SubscriptionKey::new(
                EventKind::RoundEnd,
                crate::engine::skill::rule::DefinitionKey::new(act_id, act_type),
            ),
            act_type: act_type.to_owned(),
            effect_time: 302,
            effect_condition: 0,
            args,
            raw: String::new(),
        }
    }

    #[test]
    fn use_skill_to_enemy_owns_its_configured_skill() {
        let subscriber = subscriber("UseSkillToEnemy", 759, vec![308801711, 1]);

        assert_eq!(linked(&subscriber).unwrap().skill_id, 308801711);
    }

    #[test]
    fn channel_acts_keep_their_distinct_argument_layouts() {
        assert_eq!(
            linked(&subscriber(
                "ConsumeBuffContinueChannel",
                825,
                vec![31020141, 1]
            ))
            .unwrap()
            .skill_id,
            31020141
        );
        assert_eq!(
            linked(&subscriber(
                "MonitorContinueChannel",
                1024,
                vec![31260141, 31260171]
            ))
            .unwrap()
            .skill_id,
            31260171
        );
        assert_eq!(
            linked(&subscriber(
                "ConsumeBuffAddBuffContinueChannel",
                1031,
                vec![31280151, 31280113, 0, 50]
            ))
            .unwrap()
            .skill_id,
            31280151
        );
        assert!(linked(&subscriber("UseSkillToEnemy", 825, vec![1])).is_none());
    }
}
