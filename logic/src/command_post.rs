use crate::{error::AppError, reward};
use database::db::game::command_post;
use sonettobuf::{
    CommandPostBonusAllReply, CommandPostBonusReply, CommandPostCharacterReadReply,
    CommandPostDispatchReply, CommandPostEventReadReply, CommandPostPaperReply,
    FinishCommandPostEventReply, GetCommandPostInfoReply,
};
use sqlx::SqlitePool;

const COMMAND_POST_CURRENT_VERSION_CONST_ID: i32 = 300;
const COMMAND_POST_EVENT_TYPE_DISPATCH: i32 = 4;
const COMMAND_POST_EVENT_STATE_GET_REWARD: i32 = 1;

pub struct CommandPostBonusClaim {
    pub reply: CommandPostBonusReply,
    pub rewards: reward::AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub struct CommandPostBonusAllClaim {
    pub reply: CommandPostBonusAllReply,
    pub rewards: reward::AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn get_command_post_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetCommandPostInfoReply, AppError> {
    let (info, events, tasks, catch_tasks, gain_bonus, character_state) =
        command_post::get_command_post_info(db, player_id).await?;

    Ok(GetCommandPostInfoReply {
        event_list: events.into_iter().map(Into::into).collect(),
        tasks: tasks.into_iter().map(Into::into).collect(),
        catch_tasks: catch_tasks.into_iter().map(Into::into).collect(),
        gain_bonus,
        paper: Some(info.paper),
        catch_num: Some(info.catch_num),
        character_state,
    })
}

pub async fn command_post_character_read(
    db: &SqlitePool,
    player_id: i64,
    id: Option<i32>,
) -> Result<CommandPostCharacterReadReply, AppError> {
    if let Some(id) = id {
        command_post::read_command_post_character(db, player_id, id).await?;
    }

    Ok(CommandPostCharacterReadReply { id })
}

pub async fn command_post_event_read(
    db: &SqlitePool,
    player_id: i64,
    id: Option<i32>,
) -> Result<CommandPostEventReadReply, AppError> {
    if let Some(id) = id {
        command_post::read_command_post_event(db, player_id, id).await?;
    }

    Ok(CommandPostEventReadReply { id })
}

pub async fn command_post_bonus(
    db: &SqlitePool,
    player_id: i64,
    bonus_id: Option<i32>,
) -> Result<CommandPostBonusClaim, AppError> {
    let mut rewards = reward::RewardSet::default();
    let (info, _, _, _, _, _) = command_post::get_command_post_info(db, player_id).await?;
    let mut tx = db.begin().await?;
    if let Some(id) = bonus_id.and_then(|id| claimable_bonus_id(id, info.catch_num))
        && command_post::claim_command_post_bonus_in_transaction(&mut tx, player_id, id).await?
    {
        rewards = reward::parse(&config::configs::get().copost_bonus.get(id).unwrap().bonus);
    }

    let material_changes = rewards.material_changes();
    let rewards = reward::apply_in_transaction(&mut tx, db, player_id, rewards).await?;
    tx.commit().await?;

    Ok(CommandPostBonusClaim {
        reply: CommandPostBonusReply { bonus_id },
        rewards,
        material_changes,
    })
}

pub async fn command_post_bonus_all(
    db: &SqlitePool,
    player_id: i64,
) -> Result<CommandPostBonusAllClaim, AppError> {
    let (info, _, _, _, gained, _) = command_post::get_command_post_info(db, player_id).await?;
    let mut bonus_ids = claimable_bonus_ids(info.catch_num, &gained);
    let mut reward_set = reward::RewardSet::default();
    let mut tx = db.begin().await?;

    for id in bonus_ids.iter().copied() {
        if command_post::claim_command_post_bonus_in_transaction(&mut tx, player_id, id).await? {
            reward_set.extend(reward::parse(
                &config::configs::get().copost_bonus.get(id).unwrap().bonus,
            ));
        }
    }

    bonus_ids.sort_unstable();
    let material_changes = reward_set.material_changes();
    let rewards = reward::apply_in_transaction(&mut tx, db, player_id, reward_set).await?;
    tx.commit().await?;

    Ok(CommandPostBonusAllClaim {
        reply: CommandPostBonusAllReply {
            bonus_id: bonus_ids,
        },
        rewards,
        material_changes,
    })
}

pub async fn command_post_paper(
    db: &SqlitePool,
    player_id: i64,
) -> Result<CommandPostPaperReply, AppError> {
    let paper = command_post::compose_command_post_paper(db, player_id).await?;

    Ok(CommandPostPaperReply { paper: Some(paper) })
}

pub async fn command_post_dispatch(
    db: &SqlitePool,
    player_id: i64,
    event_id: Option<i32>,
    hero_ids: Vec<i32>,
) -> Result<CommandPostDispatchReply, AppError> {
    let Some(event_id) = event_id else {
        return Ok(CommandPostDispatchReply { event: None });
    };
    let Some(event) = config::configs::get().copost_event.get(event_id) else {
        return Ok(CommandPostDispatchReply { event: None });
    };
    if event.event_type != COMMAND_POST_EVENT_TYPE_DISPATCH {
        return Ok(CommandPostDispatchReply { event: None });
    }

    let start_time = common::time::ServerTime::now_ms();
    let end_time = start_time + i64::from(event.all_time.max(0)) * 1000;
    let event = command_post::dispatch_command_post_event(
        db, player_id, event_id, &hero_ids, start_time, end_time,
    )
    .await?;

    Ok(CommandPostDispatchReply {
        event: Some(event.into()),
    })
}

pub async fn finish_command_post_event(
    db: &SqlitePool,
    player_id: i64,
    id: Option<i32>,
) -> Result<FinishCommandPostEventReply, AppError> {
    if let Some(id) = id {
        command_post::finish_command_post_event(
            db,
            player_id,
            id,
            COMMAND_POST_EVENT_STATE_GET_REWARD,
        )
        .await?;
    }

    Ok(FinishCommandPostEventReply { id })
}

fn claimable_bonus_id(id: i32, catch_num: i32) -> Option<i32> {
    let tables = config::configs::get();
    let info = tables.copost_bonus.get(id)?;
    let current_version = command_post_current_version();

    (info.version_id <= current_version && info.point_num <= catch_num).then_some(id)
}

fn claimable_bonus_ids(catch_num: i32, gained: &[i32]) -> Vec<i32> {
    let tables = config::configs::get();
    let current_version = command_post_current_version();
    let gained = gained
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>();

    tables
        .copost_bonus
        .iter()
        .filter(|bonus| bonus.version_id <= current_version)
        .filter(|bonus| bonus.point_num <= catch_num)
        .filter(|bonus| !gained.contains(&bonus.id))
        .map(|bonus| bonus.id)
        .collect()
}

fn command_post_current_version() -> i32 {
    config::configs::get()
        .copost_const
        .get(COMMAND_POST_CURRENT_VERSION_CONST_ID)
        .map(|row| row.value)
        .unwrap_or_default()
}
