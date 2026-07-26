use crate::engine::{manager::buff::ActiveBuffFeature, manager::hp::HpManager};

use super::{is_kind, registry::BuffActKind};

pub fn supports(args: &[i32]) -> bool {
    matches!(args, [max_hp_rate, 0] if *max_hp_rate > 0)
}

pub fn attack_replacement(feature: &ActiveBuffFeature, hp: &HpManager) -> Option<i32> {
    if !is_kind(
        feature,
        BuffActKind::AttrOnlyCalDamageHpReplaceAttackCalSkillDamage,
    ) {
        return None;
    }
    let [_, max_hp_rate, mode] = feature.values.as_slice() else {
        return None;
    };
    (*max_hp_rate > 0 && *mode == 0).then(|| {
        let max_hp = hp.max(feature.owner_uid).max(0);
        let replacement = max_hp * *max_hp_rate / 1000;
        if crate::engine::diagnostics::enabled(crate::engine::diagnostics::TraceArea::Damage) {
            eprintln!(
                "hp attack replacement owner={} buff={} max_hp={} rate={} output={replacement}",
                feature.owner_uid, feature.buff_id, max_hp, max_hp_rate,
            );
        }
        replacement
    })
}

pub fn skill_attack_replacement(
    feature: &ActiveBuffFeature,
    hp: &HpManager,
) -> Option<super::AttackReplacement> {
    attack_replacement(feature, hp).map(|amount| super::AttackReplacement {
        replaced_attr: crate::engine::entity::attr::AttrId::Attack,
        source_attr: crate::engine::entity::attr::AttrId::Hp,
        amount,
        formula: crate::engine::damage::DamageFormula::HpSkillDamage,
    })
}

#[cfg(test)]
mod tests {
    use sonettobuf::{Fight, FightEntityInfo, FightTeam, HeroAttribute};

    use super::*;

    #[test]
    fn configured_rate_replaces_attack_with_max_hp_scaling() {
        let mut hp = HpManager::default();
        hp.seed(&Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(1),
                    current_hp: Some(10_000),
                    attr: Some(HeroAttribute {
                        hp: Some(20_000),
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
            buff_uid: 2,
            buff_id: 31260111,
            amount: 1,
            team_type: 1,
            owner_alive: true,
            act_type: "AttrOnlyCalDamageHpReplaceAttackCalSkillDamage".to_owned(),
            effect_time: 203,
            effect_condition: 0,
            raw: "1022#200#0".to_owned(),
            values: vec![1022, 200, 0],
        };
        hp.set_max(1, 30_000);

        assert_eq!(attack_replacement(&feature, &hp), Some(6_000));
        assert!(supports(&[200, 0]));
        assert!(!supports(&[200, 1]));
    }
}
