use crate::engine::{
    entity::attr::AttrId,
    manager::{buff::ActiveBuffFeature, hp::HpManager},
};

pub fn attribute_delta(feature: &ActiveBuffFeature, attr_id: AttrId, hp: &HpManager) -> i32 {
    let mut parts = feature.raw.split('#');
    let (Some(_), Some(step), Some(raw_attrs), Some(raw_values), Some(max_steps), None) = (
        parts.next(),
        parts.next(),
        parts.next(),
        parts.next(),
        parts.next(),
        parts.next(),
    ) else {
        return 0;
    };
    let Ok(step) = step.parse::<i32>() else {
        return 0;
    };
    let Ok(max_steps) = max_steps.parse::<i32>() else {
        return 0;
    };
    if step <= 0 || max_steps <= 0 {
        return 0;
    }
    let Some(value) =
        raw_attrs
            .split(',')
            .zip(raw_values.split(','))
            .find_map(|(raw_attr, raw_value)| {
                (raw_attr.parse().ok().and_then(AttrId::from_raw) == Some(attr_id))
                    .then(|| raw_value.parse::<i32>().ok())
                    .flatten()
            })
    else {
        return 0;
    };
    let state = hp.get(feature.owner_uid);
    if state.max <= 0 {
        return 0;
    }
    let missing = (state.max - state.current).max(0) * 1000 / state.max;
    value * (missing / step).min(max_steps)
}

#[cfg(test)]
mod tests {
    use sonettobuf::{Fight, FightEntityInfo, FightTeam, HeroAttribute};

    use super::*;

    #[test]
    fn configured_attributes_scale_by_missing_hp_and_cap() {
        let mut hp = HpManager::default();
        hp.seed(&Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(1),
                    current_hp: Some(1_000),
                    attr: Some(HeroAttribute {
                        hp: Some(10_000),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        });
        let feature = ActiveBuffFeature {
            owner_uid: 1,
            source_uid: 1,
            buff_uid: 1,
            buff_id: 1,
            amount: 1,
            team_type: 1,
            owner_alive: true,
            act_type: "AttrByLostHp".into(),
            effect_time: 203,
            effect_condition: 0,
            raw: "853#100#205,206#25,25#8".into(),
            values: vec![853, 100, 205, 206, 25, 25, 8],
        };

        assert_eq!(attribute_delta(&feature, AttrId::DmgBonus, &hp), 200);
        assert_eq!(
            attribute_delta(&feature, AttrId::DmgTakenReduction, &hp),
            200
        );
    }
}
