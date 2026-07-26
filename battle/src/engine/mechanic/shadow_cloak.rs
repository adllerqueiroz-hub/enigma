use crate::engine::{
    manager::{
        BattleManagers,
        buff::ActiveBuffFeature,
        ex_point::{ExPointChange, ExPointCommand},
    },
    skill::{
        buff_act::{is_kind, raspberry::RaspberryBuffAct, registry::BuffActKind},
        rule::output::RuleOp,
    },
};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ShadowCloak {
    enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CapacityRuleGroup {
    pub owner_uid: i64,
    pub operations: Vec<RuleOp>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapacityPlanError {
    MissingSource {
        team_type: i32,
        owner_uid: i64,
        buff_uid: i64,
    },
    ConflictingSources {
        team_type: i32,
        sources: Vec<i64>,
    },
    MissingSourceOwner {
        team_type: i32,
        source_uid: i64,
    },
    UnsettledOwners {
        owner_uids: Vec<i64>,
    },
}

impl ShadowCloak {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn init_from_features(&mut self, features: &[ActiveBuffFeature]) -> Option<()> {
        let has_raspberry = features
            .iter()
            .any(|feature| feature.owner_alive && is_kind(feature, BuffActKind::Raspberry));
        if !has_raspberry {
            return None;
        }

        self.enabled = true;
        Some(())
    }
}

pub fn capacity_rule_groups(
    managers: &BattleManagers,
    features: &[ActiveBuffFeature],
    losses: &HashMap<(i64, i64), i32>,
) -> Result<Vec<CapacityRuleGroup>, CapacityPlanError> {
    let acts = features
        .iter()
        .filter_map(RaspberryBuffAct::from_feature)
        .collect::<Vec<_>>();
    let team_types =
        acts.iter()
            .map(|act| act.team_type)
            .fold(Vec::new(), |mut team_types, team_type| {
                if !team_types.contains(&team_type) {
                    team_types.push(team_type);
                }
                team_types
            });
    let mut team_contexts = HashMap::new();
    for team_type in team_types {
        let team_acts = acts
            .iter()
            .copied()
            .filter(|act| act.team_type == team_type)
            .collect::<Vec<_>>();
        if !team_acts
            .iter()
            .any(|act| losses.contains_key(&(act.owner_uid, act.buff_uid)))
        {
            continue;
        }
        let source_uid = shared_source(&team_acts, team_type)?;
        let source_act = team_acts
            .iter()
            .find(|act| act.owner_uid == source_uid)
            .ok_or(CapacityPlanError::MissingSourceOwner {
                team_type,
                source_uid,
            })?;
        let Some(source_loss) = losses
            .get(&(source_act.owner_uid, source_act.buff_uid))
            .copied()
        else {
            continue;
        };
        let shared_gain = source_act.shared_gain_from_loss(source_loss);
        if shared_gain > 0 {
            team_contexts.insert(
                team_type,
                (
                    source_uid,
                    shared_gain,
                    source_act.max_cap_from_source_hp(managers.hp.max(source_uid)),
                ),
            );
        }
    }
    let mut groups = Vec::<CapacityRuleGroup>::new();
    for act in acts {
        let Some(&(source_uid, shared_gain, max_cap)) = team_contexts.get(&act.team_type) else {
            continue;
        };
        let owner_uid = act.owner_uid;
        let Some(buff) = managers.buff.snapshot(act.owner_uid, act.buff_uid) else {
            continue;
        };
        let (current, cap) = act.capacity(&buff, max_cap);
        let next = if cap > 0 {
            (current + shared_gain).min(cap)
        } else {
            current + shared_gain
        };
        let delta = next - current;
        let origin = act.origin;
        let mut ops = Vec::with_capacity(2);
        if act.crossed_cap(current, next, cap) {
            ops.push(RuleOp::Command(
                crate::engine::skill::rule::output::BattleCommand::ExPoint(ExPointCommand::Change(
                    ExPointChange {
                        origin,
                        source_uid,
                        target_uid: source_uid,
                        delta: 1,
                        config_effect: 0,
                        effect_type: 0,
                    },
                )),
            ));
        }
        if let Some(op) = crate::engine::skill::buff_act::raspberry::capacity_rule_op(
            origin, source_uid, act, next, cap, delta,
        ) {
            ops.push(op);
        }
        if ops.is_empty() {
            continue;
        }
        if let Some(group) = groups.iter_mut().find(|group| group.owner_uid == owner_uid) {
            group.operations.extend(ops);
        } else {
            groups.push(CapacityRuleGroup {
                owner_uid,
                operations: ops,
            });
        }
    }
    Ok(groups)
}

fn shared_source(acts: &[RaspberryBuffAct], team_type: i32) -> Result<i64, CapacityPlanError> {
    let mut sources = Vec::new();
    for act in acts {
        if act.source_uid == 0 {
            return Err(CapacityPlanError::MissingSource {
                team_type,
                owner_uid: act.owner_uid,
                buff_uid: act.buff_uid,
            });
        }
        if !sources.contains(&act.source_uid) {
            sources.push(act.source_uid);
        }
    }
    match sources.as_slice() {
        [source_uid] => Ok(*source_uid),
        _ => Err(CapacityPlanError::ConflictingSources { team_type, sources }),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use sonettobuf::{BuffInfo, Fight, FightEntityInfo, FightTeam, HeroAttribute};

    use crate::engine::{
        manager::{BattleManagers, buff::ActiveBuffFeature},
        skill::rule::output::{BattleCommand, RuleOp},
    };

    use super::{ShadowCloak, capacity_rule_groups};

    #[test]
    fn initializes_only_when_raspberry_feature_exists() {
        let mut shadow = ShadowCloak::default();

        assert_eq!(shadow.init_from_features(&[]), None);
        assert!(!shadow.enabled());

        assert_eq!(
            shadow.init_from_features(&[feature(vec![1042, 100])]),
            Some(())
        );
        assert!(shadow.enabled());
    }

    #[test]
    fn capacity_rules_keep_each_owners_operations_together() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: [10_i64, 11]
                    .into_iter()
                    .enumerate()
                    .map(|(index, owner_uid)| FightEntityInfo {
                        uid: Some(owner_uid),
                        current_hp: Some(10_000),
                        attr: Some(HeroAttribute {
                            hp: Some(10_000),
                            ..Default::default()
                        }),
                        buffs: vec![BuffInfo {
                            uid: Some(20 + index as i64),
                            buff_id: Some(31250151),
                            from_uid: Some(10),
                            act_common_params: Some("0#1500".to_owned()),
                            ..Default::default()
                        }],
                        ..Default::default()
                    })
                    .collect(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = BattleManagers::seeded(&fight);
        let groups = capacity_rule_groups(
            &managers,
            &[raspberry_feature(11, 21), raspberry_feature(10, 20)],
            &HashMap::from([((10, 20), 100)]),
        )
        .unwrap();

        assert_eq!(
            groups
                .iter()
                .map(|group| group.owner_uid)
                .collect::<Vec<_>>(),
            vec![11, 10]
        );
        assert!(groups.iter().all(|group| {
            matches!(
                group.operations.as_slice(),
                [RuleOp::Command(BattleCommand::RaspberryCapacity(command))]
                    if command.target_uid == group.owner_uid
            )
        }));
    }

    #[test]
    fn capacity_rules_do_not_share_a_source_between_teams() {
        crate::test_support::init_config();
        let entity = |uid, buff_uid, source_uid| FightEntityInfo {
            uid: Some(uid),
            current_hp: Some(10_000),
            attr: Some(HeroAttribute {
                hp: Some(10_000),
                ..Default::default()
            }),
            buffs: vec![BuffInfo {
                uid: Some(buff_uid),
                buff_id: Some(31250151),
                from_uid: Some(source_uid),
                act_common_params: Some("0#1500".to_owned()),
                ..Default::default()
            }],
            ..Default::default()
        };
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![entity(10, 20, 10)],
                ..Default::default()
            }),
            defender: Some(FightTeam {
                entitys: vec![entity(30, 40, 30)],
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = BattleManagers::seeded(&fight);
        let groups = capacity_rule_groups(
            &managers,
            &[
                raspberry_feature_on(10, 20, 10, 1),
                raspberry_feature_on(30, 40, 30, 2),
            ],
            &HashMap::from([((10, 20), 100)]),
        )
        .unwrap();

        assert_eq!(
            groups
                .iter()
                .map(|group| group.owner_uid)
                .collect::<Vec<_>>(),
            vec![10]
        );
    }

    #[test]
    fn capacity_rules_reject_conflicting_team_sources() {
        crate::test_support::init_config();
        let managers = BattleManagers::default();

        assert_eq!(
            capacity_rule_groups(
                &managers,
                &[
                    raspberry_feature_on(10, 20, 10, 1),
                    raspberry_feature_on(11, 21, 11, 1),
                ],
                &HashMap::from([((10, 20), 100)]),
            ),
            Err(super::CapacityPlanError::ConflictingSources {
                team_type: 1,
                sources: vec![10, 11],
            })
        );
    }

    #[test]
    fn capacity_rules_ignore_unreached_team_source_conflicts() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(10),
                    current_hp: Some(10_000),
                    attr: Some(HeroAttribute {
                        hp: Some(10_000),
                        ..Default::default()
                    }),
                    buffs: vec![BuffInfo {
                        uid: Some(20),
                        buff_id: Some(31250151),
                        from_uid: Some(10),
                        act_common_params: Some("0#1500".to_owned()),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = BattleManagers::seeded(&fight);

        let groups = capacity_rule_groups(
            &managers,
            &[
                raspberry_feature_on(10, 20, 10, 1),
                raspberry_feature_on(30, 40, 30, 2),
                raspberry_feature_on(31, 41, 31, 2),
            ],
            &HashMap::from([((10, 20), 100)]),
        )
        .unwrap();

        assert_eq!(
            groups
                .iter()
                .map(|group| group.owner_uid)
                .collect::<Vec<_>>(),
            vec![10]
        );
    }

    fn feature(values: Vec<i32>) -> ActiveBuffFeature {
        ActiveBuffFeature {
            owner_uid: 1,
            source_uid: 1,
            buff_uid: 1,
            buff_id: 1,
            amount: 1,
            team_type: 1,
            owner_alive: true,
            act_type: "Raspberry".to_owned(),
            effect_time: 103,
            effect_condition: 0,
            raw: values
                .iter()
                .map(i32::to_string)
                .collect::<Vec<_>>()
                .join("#"),
            values,
        }
    }

    fn raspberry_feature(owner_uid: i64, buff_uid: i64) -> ActiveBuffFeature {
        raspberry_feature_on(owner_uid, buff_uid, 10, 1)
    }

    fn raspberry_feature_on(
        owner_uid: i64,
        buff_uid: i64,
        source_uid: i64,
        team_type: i32,
    ) -> ActiveBuffFeature {
        ActiveBuffFeature {
            owner_uid,
            source_uid,
            buff_uid,
            buff_id: 31250151,
            amount: 1,
            team_type,
            owner_alive: true,
            act_type: "Raspberry".to_owned(),
            effect_time: 103,
            effect_condition: 0,
            raw: "1042#100#100#700#150#203#50#211#33#1#40".to_owned(),
            values: vec![1042, 100, 100, 700, 150, 203, 50, 211, 33, 1, 40],
        }
    }
}
