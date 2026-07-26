use crate::engine::{
    entity::attr::AttrId,
    manager::{
        BattleManagers,
        hp::{HpCommand, HpHeal, HpHealKind},
    },
    skill::rule::{
        CommandOrigin, DefinitionKey, RuleDomain,
        output::{BattleCommand, RuleOp},
    },
};

const ORIGIN: CommandOrigin = CommandOrigin {
    domain: RuleDomain::Lifecycle,
    key: DefinitionKey::new(1, "DamageHealSettlement"),
};

pub fn take_settlement_rule_ops(managers: &mut BattleManagers) -> Vec<RuleOp> {
    managers
        .hp
        .take_damage_taken()
        .into_iter()
        .filter_map(|(target_uid, damage)| {
            let rate = managers
                .origin_attribute(target_uid, AttrId::DmgHeal)
                .max(0);
            let amount = damage.saturating_mul(rate) / 1000;
            (managers.hp.current(target_uid) > 0 && amount > 0).then_some(RuleOp::Command(
                BattleCommand::Hp(HpCommand::Heal(HpHeal {
                    origin: ORIGIN,
                    source_uid: target_uid,
                    target_uid,
                    amount,
                    config_effect: 0,
                    kind: HpHealKind::Normal,
                })),
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use sonettobuf::{Fight, FightEntityInfo, FightTeam, HeroAttribute, HeroSpAttribute};

    use super::*;

    #[test]
    fn damage_heal_settlement_uses_all_damage_taken_then_clears_the_round_total() {
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(10),
                    current_hp: Some(500),
                    attr: Some(HeroAttribute {
                        hp: Some(1_000),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut managers = BattleManagers::seeded(&fight);
        managers.attribute.override_sp(
            10,
            &HeroSpAttribute {
                revive: Some(100),
                ..Default::default()
            },
        );
        managers.hp.set_shield(10, 600);
        assert_eq!(managers.hp.absorb_shield(10, 600).unwrap().absorbed, 600);
        assert_eq!(managers.hp.lose(10, 50, 0).unwrap().delta, -50);

        let ops = take_settlement_rule_ops(&mut managers);
        assert!(matches!(
            ops.as_slice(),
            [RuleOp::Command(BattleCommand::Hp(HpCommand::Heal(
                HpHeal {
                    target_uid: 10,
                    amount: 65,
                    ..
                }
            )))]
        ));
        assert!(take_settlement_rule_ops(&mut managers).is_empty());
    }
}
