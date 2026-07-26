use super::*;
use database::db::game::activity218;
use sonettobuf::{Act218AcceptRewardReply, Act218FinishGameReply, Get218InfoReply};

enum GameResultType {
    Defeat,
    Draw,
    Victory,
}

impl GameResultType {
    fn from_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Self::Defeat),
            1 => Some(Self::Draw),
            2 => Some(Self::Victory),
            _ => None,
        }
    }
}

pub struct Act218RewardClaim {
    pub reply: Act218AcceptRewardReply,
    pub rewards: reward::AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn act218_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Get218InfoReply, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let state = activity218::get(db, player_id, activity_id).await?;

    Ok(info_reply(activity_id, state))
}

pub async fn finish_act218_game(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    result: Option<i32>,
    game_record: Option<String>,
) -> Result<Act218FinishGameReply, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let result = result
        .and_then(GameResultType::from_id)
        .ok_or(AppError::InvalidRequest)?;
    let points = game_points(activity_id, result)?;
    let state = activity218::finish_game(
        db,
        player_id,
        activity_id,
        points,
        max_coin(activity_id),
        game_record.as_deref().unwrap_or_default(),
    )
    .await?;

    Ok(Act218FinishGameReply {
        activity_id: Some(activity_id),
        finish_game_count: Some(state.finish_game_count.max(0) as u32),
        total_coin_num: Some(state.total_coin_num.max(0) as u32),
    })
}

pub async fn accept_act218_reward(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Act218RewardClaim, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let state = activity218::get(db, player_id, activity_id).await?;
    let reward_id = state.accepted_reward_id + 1;
    let row = reward_row(activity_id, reward_id).ok_or(AppError::InvalidRequest)?;
    if state.total_coin_num < row.coin_num {
        return Err(AppError::InvalidRequest);
    }

    let parsed = reward::parse(&row.bonus);
    let material_changes = parsed.material_changes();
    let rewards = reward::apply(db, player_id, parsed).await?;
    activity218::accept_reward(db, player_id, activity_id, reward_id).await?;

    Ok(Act218RewardClaim {
        reply: Act218AcceptRewardReply {
            activity_id: Some(activity_id),
            accepted_reward_id: Some(to_client_accepted_reward_id(reward_id)),
        },
        rewards,
        material_changes,
    })
}

fn resolve_activity_id(activity_id: Option<i32>) -> Result<i32, AppError> {
    activity_id
        .or_else(|| {
            config::configs::get()
                .activity218_control
                .iter()
                .map(|row| row.activity_id)
                .max()
        })
        .ok_or(AppError::InvalidRequest)
}

fn game_points(activity_id: i32, result: GameResultType) -> Result<i32, AppError> {
    let control = config::configs::get()
        .activity218_control
        .iter()
        .find(|row| row.activity_id == activity_id)
        .ok_or(AppError::InvalidRequest)?;

    Ok(match result {
        GameResultType::Defeat => control.lose_point,
        GameResultType::Draw => control.draw_point,
        GameResultType::Victory => control.win_point,
    })
}

fn max_coin(activity_id: i32) -> i32 {
    config::configs::get()
        .activity218_milestone_bonus
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .map(|row| row.coin_num)
        .max()
        .unwrap_or_default()
}

fn reward_row(
    activity_id: i32,
    reward_id: i32,
) -> Option<&'static config::activity218_milestone_bonus::Activity218MilestoneBonus> {
    config::configs::get()
        .activity218_milestone_bonus
        .iter()
        .find(|row| row.activity_id == activity_id && row.reward_id == reward_id)
}

fn info_reply(activity_id: i32, state: activity218::Activity218State) -> Get218InfoReply {
    Get218InfoReply {
        activity_id: Some(activity_id),
        finish_game_count: Some(state.finish_game_count.max(0) as u32),
        total_coin_num: Some(state.total_coin_num.max(0) as u32),
        accepted_reward_id: Some(to_client_accepted_reward_id(state.accepted_reward_id)),
        game_record: Some(state.game_record),
    }
}

fn to_client_accepted_reward_id(reward_id: i32) -> u32 {
    reward_id.saturating_add(1) as u32
}
