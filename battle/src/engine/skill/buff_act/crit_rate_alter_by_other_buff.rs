use crate::engine::manager::{
    buff::{ActiveBuffFeature, BuffManager},
    hp::HpManager,
};

use super::{is_kind, registry::BuffActKind};

pub fn supports(args: &[i32]) -> bool {
    matches!(args, [tracked_buff_id, _, _] if *tracked_buff_id > 0)
}

pub fn owner_conversion_rate(owner_uid: i64, buffs: &BuffManager, hp: &HpManager) -> i32 {
    buffs
        .active_features(hp)
        .iter()
        .filter(|feature| feature.owner_uid == owner_uid && feature.owner_alive)
        .map(|feature| conversion_rate(feature, buffs))
        .sum()
}

fn conversion_rate(feature: &ActiveBuffFeature, buffs: &BuffManager) -> i32 {
    if !is_kind(feature, BuffActKind::CritRateAlterByOtherBuff) {
        return 0;
    }
    let [_, tracked_buff_id, threshold, rate] = feature.values.as_slice() else {
        return 0;
    };
    let source_amount = if feature.source_uid != 0 {
        buffs.buff_id_or_type_amount(feature.source_uid, *tracked_buff_id)
    } else {
        0
    };
    let amount = if source_amount > 0 {
        source_amount
    } else {
        buffs.buff_id_or_type_amount(feature.owner_uid, *tracked_buff_id)
    };
    if amount >= (*threshold).max(0) {
        *rate
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use sonettobuf::{BuffInfo, Fight, FightEntityInfo, FightTeam};

    use super::*;

    #[test]
    fn configured_threshold_enables_excess_crit_conversion() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![
                    FightEntityInfo {
                        uid: Some(10),
                        current_hp: Some(100),
                        buffs: vec![BuffInfo {
                            uid: Some(3),
                            buff_id: Some(31280114),
                            layer: Some(4),
                            from_uid: Some(10),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    FightEntityInfo {
                        uid: Some(11),
                        current_hp: Some(100),
                        buffs: vec![BuffInfo {
                            uid: Some(2),
                            buff_id: Some(31280112),
                            duration: Some(2),
                            from_uid: Some(10),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = crate::engine::manager::BattleManagers::seeded(&fight);

        assert_eq!(owner_conversion_rate(11, &managers.buff, &managers.hp), 800);
    }
}
