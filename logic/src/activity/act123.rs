use super::*;

pub fn act123_infos(activity_id: Option<i32>) -> Get123InfosReply {
    let activity_id = activity_id.unwrap_or_else(latest_act123_activity_id);
    let tables = config::configs::get();
    let stage = tables
        .activity123_stage
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .map(|row| row.stage)
        .min()
        .unwrap_or_default();

    let mut stages = tables
        .activity123_stage
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .map(|row| Act123StageNo {
            stage: Some(row.stage),
            is_pass: Some(0),
            state: Some(0),
            act123_episodes: act123_episodes(activity_id, row.stage),
            assist_hero_info: None,
            min_round: Some(0),
            bonus_ids: Vec::new(),
        })
        .collect::<Vec<_>>();
    stages.sort_by_key(|stage| stage.stage.unwrap_or_default());

    let retail = tables
        .activity123_retail
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .min_by_key(|row| row.id)
        .map(|row| Act123RetailNo { id: Some(row.id) });

    Get123InfosReply {
        activity_id: Some(activity_id),
        stage: Some(stage),
        act123_stage: stages,
        hero_group_snapshot: Vec::new(),
        hero_group_snapshot_sub_id: Some(0),
        act123_equips: Vec::new(),
        act123_retail: retail,
        retail_hero_group_snapshot: Vec::new(),
        unlock_equip_indexs: Vec::new(),
        unlock_act123_equip_ids: Vec::new(),
        trial: None,
    }
}

pub fn act153_infos(activity_id: Option<i32>) -> Get153InfosReply {
    Get153InfosReply {
        activity_id: Some(activity_id.unwrap_or_else(latest_act153_activity_id)),
        total_count: Some(0),
        daily_count: Some(0),
    }
}

fn latest_act123_activity_id() -> i32 {
    config::configs::get()
        .activity123_stage
        .iter()
        .map(|row| row.activity_id)
        .max()
        .unwrap_or_default()
}

fn latest_act153_activity_id() -> i32 {
    config::configs::get()
        .activity153
        .iter()
        .map(|row| row.activity_id)
        .max()
        .unwrap_or_default()
}

fn act123_episodes(activity_id: i32, stage: i32) -> Vec<Act123EpisodeNo> {
    let mut episodes = config::configs::get()
        .activity123_episode
        .iter()
        .filter(|row| row.activity_id == activity_id && row.stage == stage)
        .map(|row| Act123EpisodeNo {
            layer: Some(row.layer),
            state: Some(0),
            round: Some(0),
            hero_infos: Vec::new(),
            effect_main_celebrity_equip_ids: Vec::new(),
            star: Some(0),
        })
        .collect::<Vec<_>>();
    episodes.sort_by_key(|episode| episode.layer.unwrap_or_default());

    episodes
}
