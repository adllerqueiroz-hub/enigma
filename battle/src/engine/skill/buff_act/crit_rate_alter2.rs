use crate::engine::manager::{
    buff::{ActiveBuffFeature, BuffManager},
    hp::HpManager,
};

use super::{is_kind, registry::BuffActKind};

pub fn supports(args: &[i32]) -> bool {
    matches!(args, [rate] if *rate != 0)
}

pub fn owner_conversion_rate(owner_uid: i64, buffs: &BuffManager, hp: &HpManager) -> i32 {
    buffs
        .active_features(hp)
        .iter()
        .filter(|feature| feature.owner_uid == owner_uid && feature.owner_alive)
        .map(conversion_rate)
        .sum()
}

fn conversion_rate(feature: &ActiveBuffFeature) -> i32 {
    if !is_kind(feature, BuffActKind::CritRateAlter2) {
        return 0;
    }
    let [_, rate] = feature.values.as_slice() else {
        return 0;
    };
    rate.saturating_mul(feature.amount)
}

#[cfg(test)]
mod tests {
    use sonettobuf::{BuffInfo, Fight, FightEntityInfo, FightTeam};

    use super::*;

    #[test]
    fn configured_amount_scales_excess_crit_conversion() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(10),
                    current_hp: Some(100),
                    buffs: vec![BuffInfo {
                        uid: Some(3),
                        buff_id: Some(31170034),
                        layer: Some(2),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = crate::engine::manager::BattleManagers::seeded(&fight);

        assert_eq!(
            owner_conversion_rate(10, &managers.buff, &managers.hp),
            2000
        );
    }
}
