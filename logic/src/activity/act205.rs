use super::*;
use chrono::{NaiveDateTime, TimeZone, Utc};
use sonettobuf::{Act205FinishGameReply, Act205GetGameInfoReply, Act205GetInfoReply};

pub struct Act205Claim {
    pub reply: Act205FinishGameReply,
    pub rewards: reward::AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
    pub activity_id: i32,
    pub game_type: i32,
    pub is_win: bool,
}

pub async fn act205_get_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Act205GetInfoReply, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let game_type = active_game_type(activity_id)?;

    Ok(Act205GetInfoReply {
        activity_id: Some(activity_id),
        game_type: Some(game_type),
        have_game_count: Some(game_count(db, player_id, activity_id, game_type).await?),
    })
}

pub async fn act205_get_game_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Act205GetGameInfoReply, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let game_type = active_game_type(activity_id)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act205Game).await?;
    let game_info = states
        .get(&game_type)
        .map(|(_, _, ext)| ext.clone())
        .unwrap_or_default();

    Ok(Act205GetGameInfoReply {
        activity_id: Some(activity_id),
        game_type: Some(game_type),
        game_info: Some(game_info),
    })
}

pub async fn act205_finish_game(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    game_type: Option<i32>,
    game_info: Option<String>,
    reward_id: Option<i32>,
) -> Result<Act205Claim, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let game_type = game_type.ok_or(AppError::InvalidRequest)?;
    let reward_id = reward_id.ok_or(AppError::InvalidRequest)?;
    let row = config::configs::get()
        .actvity205_mini_game_reward
        .iter()
        .find(|row| row.r#type == game_type && row.reward_id == reward_id)
        .ok_or(AppError::InvalidRequest)?;
    let have_game_count = game_count(db, player_id, activity_id, game_type).await?;
    if have_game_count <= 0 {
        return Err(AppError::InvalidRequest);
    }

    let parsed = reward::parse(&row.bonus);
    let material_changes = parsed.material_changes();
    let rewards = reward::apply(db, player_id, parsed).await?;
    let have_game_count = have_game_count - 1;
    let game_info = game_info.unwrap_or_default();
    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act205Game,
            entry_id: game_type,
            state: have_game_count,
            progress: 0,
            ext: &game_info,
        },
    )
    .await?;

    Ok(Act205Claim {
        reply: Act205FinishGameReply {
            activity_id: Some(activity_id),
            game_type: Some(game_type),
            have_game_count: Some(have_game_count),
            reward_id: Some(reward_id),
        },
        rewards,
        material_changes,
        activity_id,
        game_type,
        is_win: row.is_win,
    })
}

fn resolve_activity_id(activity_id: Option<i32>) -> Result<i32, AppError> {
    activity_id
        .or_else(|| {
            config::configs::get()
                .actvity205_stage
                .iter()
                .map(|row| row.activity_id)
                .max()
        })
        .ok_or(AppError::InvalidRequest)
}

async fn game_count(
    db: &SqlitePool,
    player_id: i64,
    activity_id: i32,
    game_type: i32,
) -> Result<i32, AppError> {
    Ok(
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act205Game)
            .await?
            .get(&game_type)
            .map(|(state, _, _)| *state)
            .unwrap_or_else(|| default_game_count(activity_id, game_type)),
    )
}

fn default_game_count(activity_id: i32, game_type: i32) -> i32 {
    config::configs::get()
        .actvity205_stage
        .iter()
        .find(|row| row.activity_id == activity_id && row.stage_id == game_type)
        .map(|row| row.times)
        .or_else(|| {
            config::configs::get()
                .activity205_enter
                .get(game_type)
                .map(|row| row.times)
        })
        .unwrap_or_default()
}

fn active_game_type(activity_id: i32) -> Result<i32, AppError> {
    let now = common::time::ServerTime::now_ms();
    config::configs::get()
        .actvity205_stage
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .find(|row| {
            let start = parse_time_ms(&row.start_time);
            let end = parse_time_ms(&row.end_time);
            start <= now && now < end
        })
        .map(|row| row.stage_id)
        .or_else(|| {
            config::configs::get()
                .actvity205_stage
                .iter()
                .filter(|row| row.activity_id == activity_id)
                .map(|row| row.stage_id)
                .min()
        })
        .ok_or(AppError::InvalidRequest)
}

fn parse_time_ms(value: &str) -> i64 {
    NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
        .map(|dt| Utc.from_utc_datetime(&dt).timestamp_millis())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn act205_reward_table_is_config_driven() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);
        let row = config::configs::get()
            .actvity205_mini_game_reward
            .iter()
            .find(|row| row.r#type == 1 && row.reward_id == 1)
            .expect("act205 reward exists");

        assert!(
            !crate::reward::parse(&row.bonus)
                .material_changes()
                .is_empty()
        );
    }
}
