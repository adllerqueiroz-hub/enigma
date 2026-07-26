use crate::engine::manager::buff::ActiveBuffFeature;

use super::{feature_kind, registry::BuffActKind};

pub fn bonus(feature: &ActiveBuffFeature) -> i32 {
    let [_, amount] = feature.values.as_slice() else {
        return 0;
    };
    if feature.owner_alive
        && matches!(
            feature_kind(feature),
            Some(BuffActKind::BuffAddAct | BuffActKind::BuffAddActLimit)
        )
    {
        *amount * feature.amount
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_configured_action_point_bonus() {
        let feature = ActiveBuffFeature {
            owner_uid: 1,
            source_uid: 1,
            buff_uid: 1,
            buff_id: 1,
            amount: 2,
            team_type: 1,
            owner_alive: true,
            act_type: "BuffAddAct".to_owned(),
            effect_time: 0,
            effect_condition: 0,
            raw: "709#1".to_owned(),
            values: vec![709, 1],
        };

        assert_eq!(bonus(&feature), 2);

        let limit = ActiveBuffFeature {
            act_type: "BuffAddActLimit".to_owned(),
            raw: "920#1".to_owned(),
            values: vec![920, 1],
            ..feature.clone()
        };
        assert_eq!(bonus(&limit), 2);

        let dead = ActiveBuffFeature {
            owner_alive: false,
            ..feature
        };
        assert_eq!(bonus(&dead), 0);
    }
}
