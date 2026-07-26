use crate::engine::{
    entity::attr::AttrId,
    manager::buff::{ActiveBuffFeature, BuffManager},
};

pub fn attribute_delta(feature: &ActiveBuffFeature, attr_id: AttrId, buffs: &BuffManager) -> i32 {
    if !super::is_kind(feature, super::registry::BuffActKind::AddAttrBySpecialCount) {
        return 0;
    }
    let [act_id, raw_attr, ..] = feature.values.as_slice() else {
        return 0;
    };
    if AttrId::from_raw(*raw_attr) != Some(attr_id) {
        return 0;
    }
    buffs
        .act_common_value(feature.owner_uid, feature.buff_uid, *act_id)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use sonettobuf::{BuffInfo, Fight, FightEntityInfo, FightTeam};

    use super::*;
    use crate::engine::manager::BattleManagers;

    #[test]
    fn configured_special_count_output_uses_its_snapshotted_signed_attribute() {
        crate::test_support::init_config();
        let managers = BattleManagers::seeded(&Fight {
            defender: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(-1),
                    current_hp: Some(100),
                    buffs: vec![BuffInfo {
                        buff_id: Some(31070141),
                        uid: Some(4),
                        from_uid: Some(10),
                        act_common_params: Some("1004#-620".to_owned()),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        });
        let feature = managers
            .buff
            .active_features(&managers.hp)
            .into_iter()
            .find(|feature| {
                super::super::is_kind(
                    feature,
                    super::super::registry::BuffActKind::AddAttrBySpecialCount,
                )
            })
            .unwrap();

        assert_eq!(
            attribute_delta(&feature, AttrId::DmgTakenReduction, &managers.buff),
            -620
        );
        assert_eq!(
            attribute_delta(&feature, AttrId::DmgBonus, &managers.buff),
            0
        );
    }
}
