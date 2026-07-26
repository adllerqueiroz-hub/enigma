use super::*;
use sonettobuf::{Answer154PuzzleReply, Get154InfosReply, PuzzleInfo};

#[derive(Clone, Copy, Eq, PartialEq)]
enum PuzzleState {
    Lock,
    UnAnswer,
    Answering,
    Solved,
    RewardGet,
}

impl PuzzleState {
    const fn id(self) -> i32 {
        match self {
            Self::Lock => 0,
            Self::UnAnswer => 1,
            Self::Answering => 2,
            Self::Solved => 3,
            Self::RewardGet => 4,
        }
    }

    const fn from_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Self::Lock),
            1 => Some(Self::UnAnswer),
            2 => Some(Self::Answering),
            3 => Some(Self::Solved),
            4 => Some(Self::RewardGet),
            _ => None,
        }
    }
}

pub struct Act154Claim {
    pub reply: Answer154PuzzleReply,
    pub rewards: Option<reward::AppliedRewards>,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn act154_infos(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Get154InfosReply, AppError> {
    let activity_id = activity_id.unwrap_or_else(default_activity_id);
    let login_count = login_count(activity_id);
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act154Puzzle).await?;

    Ok(Get154InfosReply {
        activity_id: Some(activity_id),
        login_count: Some(login_count as u32),
        infos: config_rows(activity_id)
            .into_iter()
            .map(|row| puzzle_info(row.puzzle_id, state_for_row(row, login_count, &states)))
            .collect(),
    })
}

pub async fn answer154_puzzle(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    puzzle_id: Option<u32>,
    option_id: Option<u32>,
) -> Result<Act154Claim, AppError> {
    let activity_id = activity_id.unwrap_or_else(default_activity_id);
    let puzzle_id = puzzle_id.ok_or(AppError::InvalidRequest)? as i32;
    let option_id = option_id.ok_or(AppError::InvalidRequest)? as i32;
    let row = config::configs::get()
        .activity154
        .iter()
        .find(|row| row.activity_id == activity_id && row.puzzle_id == puzzle_id)
        .ok_or(AppError::InvalidRequest)?;
    let login_count = login_count(activity_id);
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act154Puzzle).await?;
    let (state, _, ext) = state_for_row(row, login_count, &states);
    if matches!(
        PuzzleState::from_id(state),
        Some(PuzzleState::Lock | PuzzleState::RewardGet)
    ) {
        return Err(AppError::InvalidRequest);
    }

    let mut records = answer_records(&ext);
    if !records.contains(&option_id) {
        records.push(option_id);
    }
    let next_state = if option_id == row.answer_id {
        PuzzleState::RewardGet
    } else {
        PuzzleState::UnAnswer
    };
    let ext = records
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join("#");

    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act154Puzzle,
            entry_id: puzzle_id,
            state: next_state.id(),
            progress: 0,
            ext: &ext,
        },
    )
    .await?;

    let (rewards, material_changes) = if option_id == row.answer_id {
        let parsed = reward::parse(&row.bonus);
        let material_changes = parsed.material_changes();
        let rewards = reward::apply(db, player_id, parsed).await?;
        (Some(rewards), material_changes)
    } else {
        (None, Vec::new())
    };

    Ok(Act154Claim {
        reply: Answer154PuzzleReply {
            activity_id: Some(activity_id),
            info: Some(puzzle_info(puzzle_id, (next_state.id(), 0, ext))),
        },
        rewards,
        material_changes,
    })
}

fn default_activity_id() -> i32 {
    config::configs::get()
        .activity154
        .iter()
        .map(|row| row.activity_id)
        .max()
        .unwrap_or_default()
}

fn login_count(activity_id: i32) -> i32 {
    config_rows(activity_id)
        .into_iter()
        .map(|row| row.day)
        .max()
        .unwrap_or(1)
}

fn config_rows(activity_id: i32) -> Vec<&'static config::activity154::Activity154> {
    let mut rows = config::configs::get()
        .activity154
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| row.day);
    rows
}

fn state_for_row(
    row: &config::activity154::Activity154,
    login_count: i32,
    states: &activity_state::ActivityStates,
) -> (i32, i32, String) {
    states.get(&row.puzzle_id).cloned().unwrap_or_else(|| {
        let state = if row.day <= login_count {
            PuzzleState::UnAnswer
        } else {
            PuzzleState::Lock
        };
        (state.id(), 0, String::new())
    })
}

fn puzzle_info(puzzle_id: i32, state: (i32, i32, String)) -> PuzzleInfo {
    PuzzleInfo {
        puzzle_id: Some(puzzle_id as u32),
        state: Some(state.0 as u32),
        answer_records: answer_records(&state.2)
            .into_iter()
            .map(|id| id as u32)
            .collect(),
    }
}

fn answer_records(ext: &str) -> Vec<i32> {
    ext.split('#')
        .filter_map(|part| part.parse::<i32>().ok())
        .collect()
}
