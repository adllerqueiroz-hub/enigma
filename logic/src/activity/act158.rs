use sonettobuf::{Act158Info, Get158InfosReply};

pub fn act158_infos(activity_id: Option<i32>) -> Get158InfosReply {
    let activity_id = activity_id.unwrap_or_else(default_activity_id);

    Get158InfosReply {
        info: Some(Act158Info {
            activity_id: Some(activity_id),
            open_challenge: Some(false),
            curr_difficulty: Some(0),
            pass_difficulty: Vec::new(),
            enter_difficulty: Vec::new(),
            pass_challenge_ids: Vec::new(),
        }),
    }
}

fn default_activity_id() -> i32 {
    config::configs::get()
        .activity158_challenge
        .iter()
        .map(|row| row.activity_id)
        .max()
        .unwrap_or_default()
}
