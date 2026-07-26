use crate::engine::{
    entity::attr::AttrId, manager::buff::ActiveBuffFeature, skill::target::EntityDamageType,
};

pub fn attribute_delta(
    feature: &ActiveBuffFeature,
    damage_type: EntityDamageType,
    attr_id: AttrId,
) -> i32 {
    if !super::is_kind(feature, super::registry::BuffActKind::AttrByDamageType) {
        return 0;
    }
    let [_, raw_damage_type, raw_attr, delta] = feature.values.as_slice() else {
        return 0;
    };
    if damage_type == EntityDamageType::Unknown
        || EntityDamageType::from_wire(*raw_damage_type) != damage_type
        || AttrId::from_raw(*raw_attr) != Some(attr_id)
    {
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
            amount: 1,
            team_type: 1,
            owner_alive: true,
            act_type: "AttrByDmgType".into(),
            effect_time: 201,
            effect_condition: 0,
            raw: "752#2#203#200".into(),
            values: vec![752, 2, 203, 200],
        }
    }

    #[test]
    fn song_of_generosity_grants_critical_damage_only_to_mental_characters() {
        assert_eq!(
            attribute_delta(&feature(), EntityDamageType::Mental, AttrId::CriticalDmg),
            200
        );
        assert_eq!(
            attribute_delta(&feature(), EntityDamageType::Reality, AttrId::CriticalDmg),
            0
        );
        assert_eq!(
            attribute_delta(&feature(), EntityDamageType::Mental, AttrId::CriticalRate),
            0
        );
    }
}
