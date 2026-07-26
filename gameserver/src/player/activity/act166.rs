use sonettobuf::{BaseNo, Get166InfosReply, InformationNo, TalentNo, TeachNo, TrainNo};

pub fn act166_infos(activity_id: Option<i32>) -> Get166InfosReply {
    let activity_id = activity_id.unwrap_or_else(default_activity_id);
    let tables = config::configs::get();

    Get166InfosReply {
        activity_id: Some(activity_id),
        bases: tables
            .activity166_base
            .iter()
            .filter(|row| row.activity_id == activity_id)
            .map(|row| BaseNo {
                id: Some(row.base_id),
                is_enter: Some(false),
                max_score: Some(0),
            })
            .collect(),
        trains: tables
            .activity166_train
            .iter()
            .filter(|row| row.activity_id == activity_id)
            .map(|row| TrainNo {
                id: Some(row.train_id),
                pass_count: Some(0),
            })
            .collect(),
        teachs: tables
            .activity166_teach
            .iter()
            .map(|row| TeachNo {
                id: Some(row.teach_id),
                pass_count: Some(0),
            })
            .collect(),
        is_finish_teach: Some(false),
        talents: tables
            .activity166_talent
            .iter()
            .filter(|row| row.activity_id == activity_id)
            .map(|row| TalentNo {
                id: Some(row.talent_id),
                level: Some(1),
                skill_ids: Vec::new(),
            })
            .collect(),
        information: Some(InformationNo {
            infos: Vec::new(),
            bonus_ids: Vec::new(),
        }),
        base_hero_group_snapshot: Vec::new(),
        train_hero_group_snapshot: None,
    }
}

fn default_activity_id() -> i32 {
    config::configs::get()
        .activity166_base
        .iter()
        .map(|row| row.activity_id)
        .max()
        .unwrap_or_default()
}
