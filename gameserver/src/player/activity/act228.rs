use super::*;

pub fn act228_info(activity_id: Option<i32>) -> GetAct228InfoReply {
    let row = act228_config(activity_id);
    let activity_id = act228_activity_id(activity_id, row);

    GetAct228InfoReply {
        activity_id: Some(activity_id),
        info: Some(Act228Info {
            normal_flip_count: Some(0),
            reward_ids: row.map(act228_reward_ids).unwrap_or_default(),
            grid_states: act228_grid_states(row),
            get_final_bonus: Some(false),
        }),
    }
}

pub fn act228_flip_grid(activity_id: Option<i32>) -> Act228FlipGridGridReply {
    let row = act228_config(activity_id);

    Act228FlipGridGridReply {
        activity_id: Some(act228_activity_id(activity_id, row)),
        grid_states: act228_grid_states(row),
        bonuses: Vec::new(),
    }
}

pub fn act228_get_final_bonus(activity_id: Option<i32>) -> Act228GetFinalBonusReply {
    let row = act228_config(activity_id);

    Act228GetFinalBonusReply {
        activity_id: Some(act228_activity_id(activity_id, row)),
        bonuses: Vec::new(),
    }
}

fn act228_config(activity_id: Option<i32>) -> Option<&'static config::activity228::Activity228> {
    let rows = config::configs::get().activity228.iter();
    match activity_id {
        Some(activity_id) => rows.into_iter().find(|row| row.activity_id == activity_id),
        None => rows.into_iter().max_by_key(|row| row.activity_id),
    }
}

fn act228_activity_id(
    activity_id: Option<i32>,
    row: Option<&config::activity228::Activity228>,
) -> i32 {
    row.map(|row| row.activity_id)
        .or(activity_id)
        .unwrap_or_else(latest_act228_activity_id)
}

fn act228_grid_states(row: Option<&config::activity228::Activity228>) -> Vec<i32> {
    vec![0; row.map(act228_grid_count).unwrap_or_default()]
}

fn act228_grid_count(row: &config::activity228::Activity228) -> usize {
    row.row.max(0) as usize * row.column.max(0) as usize
}

fn act228_reward_ids(row: &config::activity228::Activity228) -> Vec<i32> {
    row.reward
        .split('|')
        .filter_map(|entry| entry.split('#').next())
        .filter_map(|id| id.parse::<i32>().ok())
        .filter(|id| *id > 0)
        .collect()
}
