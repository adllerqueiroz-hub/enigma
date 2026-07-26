use std::collections::BTreeMap;

use crate::engine::{
    entity::attr::AttrId,
    manager::{
        buff::{ActiveBuffFeature, BuffManager, CommandOrigin},
        hp::HpManager,
    },
    skill::buff_act::{feature_command_origin, is_kind, registry::BuffActKind},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraTransfer {
    pub buff_uid: i64,
    pub consumed_layers: i32,
    pub progress: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TeamAttributeRule {
    pub dynamic_attr: AttrId,
    pub per_recorded_stack: i32,
    pub fixed_attr: AttrId,
    pub fixed_value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransferRecord {
    pub origin: CommandOrigin,
    pub owner_uid: i64,
    pub buff_uid: i64,
    pub act_id: i32,
    pub delta: i32,
}

pub fn supports_extra_value(args: &[i32]) -> bool {
    matches!(args, [multiplier] if *multiplier > 0)
}

pub fn supports_team_attribute(args: &[i32]) -> bool {
    team_attribute_rule(args).is_some()
}

pub fn supports_record(args: &[i32]) -> bool {
    matches!(args, [cap] if *cap > 0)
}

pub fn team_attribute_rule(args: &[i32]) -> Option<TeamAttributeRule> {
    let [dynamic_attr, per_recorded_stack, fixed_attr, fixed_value] = args else {
        return None;
    };
    (*per_recorded_stack > 0 && *fixed_value >= 0).then_some(TeamAttributeRule {
        dynamic_attr: AttrId::from_raw(*dynamic_attr)?,
        per_recorded_stack: *per_recorded_stack,
        fixed_attr: AttrId::from_raw(*fixed_attr)?,
        fixed_value: *fixed_value,
    })
}

pub fn transfer_records(
    buffs: &BuffManager,
    hp: &HpManager,
    team_type: i32,
    contributor_uid: i64,
    amount: i32,
) -> Vec<TransferRecord> {
    buffs
        .active_features(hp)
        .into_iter()
        .filter(|feature| {
            feature.team_type == team_type
                && feature.owner_uid != contributor_uid
                && feature.owner_alive
                && is_kind(feature, BuffActKind::RecordTeamExElectricTransConsumeValue)
        })
        .filter_map(|feature| {
            let [act_id, cap] = feature.values.as_slice() else {
                return None;
            };
            let delta = amount
                .max(0)
                .min(cap.saturating_sub(buffs.act_value(feature.buff_uid, *act_id)));
            (delta > 0).then_some(TransferRecord {
                origin: feature_command_origin(&feature)?,
                owner_uid: feature.owner_uid,
                buff_uid: feature.buff_uid,
                act_id: *act_id,
                delta,
            })
        })
        .collect()
}

pub fn team_damage_multiplier_delta(buffs: &BuffManager, hp: &HpManager, owner_uid: i64) -> i32 {
    let team_type = buffs.team_type(owner_uid);
    buffs
        .active_features(hp)
        .into_iter()
        .filter(|feature| {
            Some(feature.team_type) == team_type
                && feature.owner_alive
                && is_kind(feature, BuffActKind::TeamExElectricTransConsumeValueAttr)
        })
        .filter_map(|feature| {
            let [act_id, args @ ..] = feature.values.as_slice() else {
                return None;
            };
            let rule = team_attribute_rule(args)?;
            (rule.dynamic_attr == AttrId::DmgBonus)
                .then(|| buffs.grant_value(feature.buff_uid, *act_id))
                .flatten()
        })
        .sum()
}

pub fn extra_transfers(buffs: &BuffManager, hp: &HpManager, owner_uid: i64) -> Vec<ExtraTransfer> {
    let mut by_buff = BTreeMap::<i64, ExtraTransfer>::new();
    for transfer in buffs
        .active_features(hp)
        .into_iter()
        .filter(|feature| feature.owner_uid == owner_uid && feature.owner_alive)
        .filter_map(extra_transfer)
    {
        by_buff
            .entry(transfer.buff_uid)
            .and_modify(|existing| {
                existing.progress = existing.progress.saturating_add(transfer.progress);
            })
            .or_insert(transfer);
    }
    by_buff.into_values().collect()
}

fn extra_transfer(feature: ActiveBuffFeature) -> Option<ExtraTransfer> {
    if !is_kind(&feature, BuffActKind::ExtraValueElectricTransform) {
        return None;
    }
    let multiplier = *feature.values.get(1)?;
    let consumed_layers = feature.amount.max(0);
    let progress = consumed_layers.saturating_mul(multiplier);
    (progress > 0).then_some(ExtraTransfer {
        buff_uid: feature.buff_uid,
        consumed_layers,
        progress,
    })
}

#[cfg(test)]
mod tests {
    use sonettobuf::{BuffInfo, Fight, FightEntityInfo, FightTeam};

    use super::*;
    use crate::engine::manager::BattleManagers;

    #[test]
    fn configured_extra_value_uses_the_carrier_layer() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(10),
                    team_type: Some(1),
                    current_hp: Some(100),
                    buffs: vec![BuffInfo {
                        buff_id: Some(31430141),
                        uid: Some(2),
                        layer: Some(6),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = BattleManagers::seeded(&fight);

        assert_eq!(
            extra_transfers(&managers.buff, &managers.hp, 10),
            vec![ExtraTransfer {
                buff_uid: 2,
                consumed_layers: 6,
                progress: 6,
            }]
        );
    }

    #[test]
    fn recorded_team_damage_bonus_keeps_its_separate_multiplier_lane() {
        use crate::engine::{
            manager::buff::{BuffAccumulateActValue, BuffCommand, BuffGrant},
            skill::rule::{DefinitionKey, RuleDomain},
        };

        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![
                    FightEntityInfo {
                        uid: Some(10),
                        team_type: Some(1),
                        current_hp: Some(100),
                        buffs: vec![BuffInfo {
                            buff_id: Some(31430145),
                            uid: Some(2),
                            from_uid: Some(10),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    FightEntityInfo {
                        uid: Some(11),
                        team_type: Some(1),
                        current_hp: Some(100),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut managers = BattleManagers::seeded(&fight);
        managers
            .execute_buff(BuffCommand::AccumulateActValue(BuffAccumulateActValue {
                origin: CommandOrigin {
                    domain: RuleDomain::BuffAct,
                    key: DefinitionKey::new(1128, "RecordTeamExElectricTransConsumeValue"),
                },
                target_uid: 10,
                buff_uid: 2,
                act_id: 1128,
                delta: 30,
            }))
            .unwrap();
        managers
            .execute_buff(BuffCommand::Grant(BuffGrant {
                origin: CommandOrigin {
                    domain: RuleDomain::Behavior,
                    key: DefinitionKey::new(1, "AddBuff"),
                },
                source_uid: 10,
                target_uid: 10,
                buff_id: 31430131,
                amount: None,
                occurrences: 1,
                child_uid_reservations: 0,
            }))
            .unwrap();

        assert_eq!(
            team_damage_multiplier_delta(&managers.buff, &managers.hp, 10),
            600
        );
        assert_eq!(
            team_damage_multiplier_delta(&managers.buff, &managers.hp, 11),
            600
        );
    }
}
