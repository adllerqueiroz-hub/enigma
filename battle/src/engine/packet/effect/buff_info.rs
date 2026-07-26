use super::*;

impl EffectPacket {
    pub fn buff_act_info(
        target_uid: i64,
        buff_uid: i64,
        act_id: i32,
        params: Vec<i32>,
    ) -> ActEffect {
        Self::buff_act_info_with_team(target_uid, buff_uid, act_id, params, 0)
    }

    pub fn buff_act_info_with_team(
        target_uid: i64,
        buff_uid: i64,
        act_id: i32,
        params: Vec<i32>,
        team_type: i32,
    ) -> ActEffect {
        Self::buff_act_info_with_team_and_str(
            target_uid,
            buff_uid,
            act_id,
            params,
            String::new(),
            team_type,
        )
    }

    pub fn buff_act_info_with_team_and_str(
        target_uid: i64,
        buff_uid: i64,
        act_id: i32,
        params: Vec<i32>,
        str_param: String,
        team_type: i32,
    ) -> ActEffect {
        ActEffect {
            target_id: Some(target_uid),
            effect_type: Some(EffectType::Buffactinfoupdate as i32),
            buff_act_info: Some(BuffActInfo {
                act_id: Some(act_id),
                param: params,
                str_param: Some(str_param),
            }),
            effect_num: Some(0),
            config_effect: Some(0),
            buff_act_id: Some(0),
            reserve_id: Some(buff_uid),
            team_type: Some(team_type),
            effect_num1: Some(0),
            ..Default::default()
        }
    }
}
