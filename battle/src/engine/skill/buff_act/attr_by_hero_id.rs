use crate::engine::{entity::attr::AttrId, manager::buff::ActiveBuffFeature};

pub fn attribute_delta(feature: &ActiveBuffFeature, model_id: i32, attr_id: AttrId) -> i32 {
    if !super::is_kind(feature, super::registry::BuffActKind::AttrByHeroId) {
        return 0;
    }
    let [_, raw_attr, delta, model_ids @ ..] = feature.values.as_slice() else {
        return 0;
    };
    if AttrId::from_raw(*raw_attr) != Some(attr_id) || !model_ids.contains(&model_id) {
        return 0;
    }
    delta.saturating_mul(feature.amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feature() -> ActiveBuffFeature {
        ActiveBuffFeature {
            owner_uid: 10,
            source_uid: 10,
            buff_uid: 20,
            buff_id: 30,
            amount: 2,
            team_type: 1,
            owner_alive: true,
            act_type: "AttrByHeroId".into(),
            effect_time: 0,
            effect_condition: 0,
            raw: "10002#213#280#3122,3123,3124".into(),
            values: vec![10002, 213, 280, 3122, 3123, 3124],
        }
    }

    #[test]
    fn matching_hero_receives_the_configured_attribute_per_layer() {
        assert_eq!(attribute_delta(&feature(), 3124, AttrId::Penetration), 560);
        assert_eq!(attribute_delta(&feature(), 3125, AttrId::Penetration), 0);
        assert_eq!(attribute_delta(&feature(), 3124, AttrId::DmgBonus), 0);
    }
}
