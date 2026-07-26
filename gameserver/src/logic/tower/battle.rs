use crate::{error::AppError, player::battle::ActiveBattle};
use database::{db::game::tower as tower_db, models::game::tower::TowerType};
use sonettobuf::{
    GetTowerInfoReply, TowerBattleFinishPush, TowerMopUpReply, TowerResetSubEpisodeReply,
};
use sqlx::SqlitePool;

pub async fn info(db: &SqlitePool, player_id: i64) -> Result<GetTowerInfoReply, AppError> {
    let (info, opens, towers, assist_bosses) = tower_db::get_tower_info(db, player_id).await?;
    Ok(GetTowerInfoReply {
        tower_opens: opens.into_iter().map(Into::into).collect(),
        towers,
        assist_bosses: assist_bosses.into_iter().map(Into::into).collect(),
        mop_up_times: Some(info.mop_up_times),
        trial_hero_season: Some(info.trial_hero_season),
    })
}

pub async fn mop_up(
    db: &SqlitePool,
    player_id: i64,
    times: i32,
) -> Result<
    (
        TowerMopUpReply,
        crate::logic::reward::AppliedRewards,
        Vec<(u32, u32, i32)>,
    ),
    AppError,
> {
    if times <= 0 {
        return Err(AppError::InvalidRequest);
    }
    let pass_layer = tower_db::tower_pass_layer(db, player_id, TowerType::Normal.id(), 0)
        .await?
        .ok_or(AppError::InvalidRequest)?;
    let reward =
        mop_up_reward(config::configs::get(), pass_layer).ok_or(AppError::InvalidRequest)?;
    let mut tx = db.begin().await?;
    let remaining = tower_db::consume_mop_up_times_in_transaction(&mut tx, player_id, times)
        .await?
        .ok_or(AppError::InvalidRequest)?;
    let mut rewards = crate::logic::reward::parse(reward);
    rewards.scale(times);
    let material_changes = rewards.material_changes();
    let applied =
        crate::logic::reward::apply_in_transaction(&mut tx, db, player_id, rewards).await?;
    tx.commit().await?;

    Ok((
        TowerMopUpReply {
            times: Some(times),
            mop_up_times: Some(remaining),
        },
        applied,
        material_changes,
    ))
}

pub(super) fn mop_up_reward(tables: &config::GameDB, pass_layer: i32) -> Option<&str> {
    tables
        .tower_mop_up
        .iter()
        .filter(|row| row.layer_num <= pass_layer)
        .max_by_key(|row| row.layer_num)
        .map(|row| row.reward.as_str())
}

pub async fn reset_sub_episode(
    db: &SqlitePool,
    player_id: i64,
    tower_type: i32,
    tower_id: i32,
    layer_id: i32,
    episode_id: i32,
) -> Result<TowerResetSubEpisodeReply, AppError> {
    let (layer_info, history_high_score) =
        tower_db::reset_sub_episode(db, player_id, tower_type, tower_id, layer_id, episode_id)
            .await?
            .ok_or(AppError::InvalidRequest)?;
    Ok(TowerResetSubEpisodeReply {
        tower_type: Some(tower_type),
        tower_id: Some(tower_id),
        layer_id: None,
        sub_episode: Some(episode_id),
        layer_info: Some(layer_info),
        history_high_score: Some(history_high_score),
        params: None,
    })
}

pub async fn abort_finish_push(
    db: &SqlitePool,
    player_id: i64,
    active: &ActiveBattle,
) -> Result<Option<TowerBattleFinishPush>, AppError> {
    let Some(tower_type) = active.tower_type else {
        return Ok(None);
    };
    let tower_id = active.tower_id.unwrap_or_default();
    let layer_id = active.layer_id.unwrap_or_default();
    let layer = tower_db::layer_info(db, player_id, tower_type, tower_id, layer_id).await?;
    let history_high_score =
        tower_db::tower_history_high_score(db, player_id, tower_type, tower_id)
            .await?
            .unwrap_or_default();

    Ok(Some(TowerBattleFinishPush {
        r#type: Some(tower_type),
        tower_id: Some(tower_id),
        layer_id: Some(layer_id),
        difficulty: Some(active.difficulty.unwrap_or_default()),
        score: Some(0),
        boss_level: Some(active.assist_boss_level.unwrap_or_default()),
        team_level: Some(active.team_level.unwrap_or_default()),
        layer,
        history_high_score: Some(history_high_score),
        params: None,
    }))
}
