use sonettobuf::BuffActInfo;

use crate::engine::{
    manager::{
        BattleManagers,
        buff::{ActiveBuffFeature, BuffActInfoMarkerResult, BuffCommand, BuffSetState, BuffStatus},
    },
    skill::{
        buff_act::registry::BuffActKind,
        rule::{
            SetupStage,
            output::{BattleCommand, RuleOp},
        },
    },
};

pub fn supports(args: &[i32]) -> bool {
    matches!(
        args,
        [status, count]
            if BuffStatus::from_id(*status) == BuffStatus::Control && *count > 0
    )
}

pub fn setup_rule_ops(
    managers: &BattleManagers,
    feature: &ActiveBuffFeature,
    stage: SetupStage,
) -> Option<Vec<RuleOp>> {
    if stage != SetupStage::RoundStart || !super::is_kind(feature, BuffActKind::TeamImmunityTimes) {
        return Some(Vec::new());
    }
    let [act_id, status, count] = feature.values.as_slice() else {
        return None;
    };
    if !supports(&[*status, *count]) {
        return None;
    }

    let mut buff = managers
        .buff
        .snapshot(feature.owner_uid, feature.buff_uid)?;
    if let Some(info) = buff
        .act_info
        .iter_mut()
        .find(|info| info.act_id == Some(*act_id))
    {
        info.param = vec![*count];
        info.str_param = Some(String::new());
    } else {
        buff.act_info.push(BuffActInfo {
            act_id: Some(*act_id),
            param: vec![*count],
            str_param: Some(String::new()),
        });
    }
    let origin = super::feature_command_origin(feature)?;
    Some(vec![
        RuleOp::Command(BattleCommand::Buff(BuffCommand::SetInternalState(
            BuffSetState {
                origin,
                target_uid: feature.owner_uid,
                buff_uid: feature.buff_uid,
                ex_info: None,
                params: None,
                act_info: Some(buff.act_info),
            },
        ))),
        RuleOp::BuffActInfoMarker(BuffActInfoMarkerResult {
            target_uid: feature.owner_uid,
            buff_uid: feature.buff_uid,
            act_id: *act_id,
            params: vec![*count],
            str_param: Some(String::new()),
            team_type: 0,
        }),
    ])
}

#[cfg(test)]
mod tests {
    use sonettobuf::{BuffInfo, Fight, FightEntityInfo, FightTeam};

    use super::*;

    #[test]
    fn round_start_resets_the_configured_team_control_immunity_budget() {
        crate::test_support::init_config();
        let fight = Fight {
            version: Some(7),
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(10),
                    team_type: Some(1),
                    current_hp: Some(1),
                    buffs: vec![BuffInfo {
                        buff_id: Some(31430144),
                        uid: Some(20),
                        from_uid: Some(10),
                        act_info: vec![BuffActInfo {
                            act_id: Some(1126),
                            param: vec![1],
                            str_param: Some(String::new()),
                        }],
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let managers = BattleManagers::seeded(&fight);
        let feature = managers
            .buff
            .active_features(&managers.hp)
            .into_iter()
            .find(|feature| super::super::is_kind(feature, BuffActKind::TeamImmunityTimes))
            .unwrap();

        let ops = setup_rule_ops(&managers, &feature, SetupStage::RoundStart).unwrap();

        assert!(matches!(
            ops.as_slice(),
            [
                RuleOp::Command(BattleCommand::Buff(BuffCommand::SetInternalState(
                    BuffSetState {
                        act_info,
                        ..
                    }
                ))),
                RuleOp::BuffActInfoMarker(BuffActInfoMarkerResult {
                    act_id: 1126,
                    params,
                    ..
                })
            ] if act_info.as_ref().unwrap()[0].param == [4] && params == &[4]
        ));
    }
}
