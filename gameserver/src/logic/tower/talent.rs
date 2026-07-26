use crate::error::AppError;
use database::{db::game::tower as tower_db, models::game::tower::TowerConstId};
use sqlx::SqlitePool;

pub async fn activate_talent(
    db: &SqlitePool,
    player_id: i64,
    boss_id: i32,
    talent_id: i32,
) -> Result<i32, AppError> {
    let tables = config::configs::get();
    let talent = tables
        .tower_assist_talent
        .iter()
        .find(|row| row.boss_id == boss_id && row.node_id == talent_id)
        .ok_or(AppError::InvalidRequest)?;
    let plan = tower_db::active_talent_plan(db, player_id, boss_id)
        .await?
        .ok_or(AppError::InvalidRequest)?;
    if plan.talent_ids.contains(&talent_id) || plan.talent_point < talent.consume {
        return Err(AppError::InvalidRequest);
    }
    if talent.node_group != 0
        && tables.tower_assist_talent.iter().any(|row| {
            row.boss_id == boss_id
                && row.node_group == talent.node_group
                && plan.talent_ids.contains(&row.node_id)
        })
    {
        return Err(AppError::InvalidRequest);
    }
    if !prerequisites_met(&talent.pre_node_ids, &plan.talent_ids) {
        return Err(AppError::InvalidRequest);
    }

    let remaining = plan.talent_point - talent.consume;
    if !tower_db::activate_talent(
        db,
        player_id,
        boss_id,
        plan.plan_id,
        talent_id,
        plan.talent_point,
        remaining,
    )
    .await?
    {
        return Err(AppError::InvalidRequest);
    }
    Ok(remaining)
}

pub async fn reset_talent(
    db: &SqlitePool,
    player_id: i64,
    boss_id: i32,
    talent_id: i32,
) -> Result<i32, AppError> {
    let tables = config::configs::get();
    let plan = tower_db::active_talent_plan(db, player_id, boss_id)
        .await?
        .ok_or(AppError::InvalidRequest)?;

    let (reset_id, talent_point) = if talent_id == 0 {
        let total = tables
            .tower_assist_develop
            .iter()
            .filter(|row| row.boss_id == boss_id && row.level <= plan.boss_level)
            .map(|row| row.talent_point)
            .sum();
        (None, total)
    } else {
        let talent = tables
            .tower_assist_talent
            .iter()
            .find(|row| row.boss_id == boss_id && row.node_id == talent_id)
            .filter(|_| plan.talent_ids.contains(&talent_id))
            .ok_or(AppError::InvalidRequest)?;
        let has_active_child = tables.tower_assist_talent.iter().any(|row| {
            row.boss_id == boss_id
                && plan.talent_ids.contains(&row.node_id)
                && prerequisite_ids(&row.pre_node_ids).any(|id| id == talent_id)
        });
        if has_active_child {
            return Err(AppError::InvalidRequest);
        }
        (Some(talent_id), plan.talent_point + talent.consume)
    };

    if !tower_db::reset_talent(
        db,
        player_id,
        boss_id,
        plan.plan_id,
        reset_id,
        plan.talent_point,
        talent_point,
    )
    .await?
    {
        return Err(AppError::InvalidRequest);
    }
    Ok(talent_point)
}

pub async fn change_talent_plan(
    db: &SqlitePool,
    player_id: i64,
    boss_id: i32,
    plan_id: i32,
) -> Result<(), AppError> {
    let tables = config::configs::get();
    let custom_plan_count = tables
        .tower_const
        .get(TowerConstId::CustomTalentPlanCount.id())
        .and_then(|row| row.value.parse().ok())
        .unwrap_or_default();
    let valid = (1..=custom_plan_count).contains(&plan_id)
        || tables
            .tower_talent_plan
            .iter()
            .any(|row| row.boss_id == boss_id && row.plan_id == plan_id);
    if !valid || !tower_db::change_talent_plan(db, player_id, boss_id, plan_id).await? {
        return Err(AppError::InvalidRequest);
    }
    Ok(())
}

pub async fn rename_active_talent_plan(
    db: &SqlitePool,
    player_id: i64,
    boss_id: i32,
    plan_name: &str,
) -> Result<(), AppError> {
    let plan_name = plan_name.trim();
    if plan_name.is_empty()
        || !tower_db::rename_active_talent_plan(db, player_id, boss_id, plan_name).await?
    {
        return Err(AppError::InvalidRequest);
    }
    Ok(())
}

fn prerequisites_met(value: &str, active: &[i32]) -> bool {
    if value.is_empty() {
        return true;
    }
    if value.contains('&') {
        prerequisite_ids(value).all(|id| active.contains(&id))
    } else {
        prerequisite_ids(value).any(|id| active.contains(&id))
    }
}

fn prerequisite_ids(value: &str) -> impl Iterator<Item = i32> + '_ {
    value
        .split(['#', '&'])
        .filter_map(|value| value.parse().ok())
}
