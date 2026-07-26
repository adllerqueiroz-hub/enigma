use super::*;

pub async fn buy_levels(
    db: &SqlitePool,
    player_id: i64,
    id: Option<i32>,
    num: Option<i32>,
) -> Result<BpLevelPurchase, AppError> {
    let bp_id = id
        .or_else(task_db::current_battle_pass_id)
        .ok_or(AppError::InvalidRequest)?;
    let num = num.filter(|num| *num > 0).ok_or(AppError::InvalidRequest)?;
    let tables = config::configs::get();
    let bp = tables
        .bp
        .iter()
        .find(|bp| bp.bp_id == bp_id)
        .ok_or(AppError::InvalidRequest)?;
    let state = battle_pass::get_or_create_state(db, player_id, bp_id).await?;
    let max_level = i32::try_from(
        tables
            .bp_lv_bonus
            .iter()
            .filter(|bonus| bonus.bp_id == bp_id)
            .count(),
    )
    .map_err(|_| AppError::InvalidRequest)?;
    let current_level = state.score / bp.exp_level_up.max(1);
    if num > max_level.saturating_sub(current_level) {
        return Err(AppError::InvalidRequest);
    }

    let cost = tables
        .r#const
        .get(BP_BUY_LEVEL_COST_CONFIG_ID)
        .and_then(|row| level_purchase_cost(&row.value, num))
        .ok_or(AppError::InvalidRequest)?;
    let score_delta = bp
        .exp_level_up
        .checked_mul(num)
        .ok_or(AppError::InvalidRequest)?;
    let score = battle_pass::buy_levels(db, player_id, bp_id, cost.0, cost.1, score_delta)
        .await?
        .ok_or(AppError::InsufficientCurrency)?;

    Ok(BpLevelPurchase {
        reply: BpBuyLevelReply {
            id: Some(bp_id),
            score: Some(score),
        },
        currency_change: (cost.0, -cost.1),
        material_change: (2, cost.0 as u32, -cost.1),
    })
}

pub(super) fn level_purchase_cost(value: &str, levels: i32) -> Option<(i32, i32)> {
    let costs = reward::parse(value);
    let [(currency_id, unit_cost)] = costs.currencies.as_slice() else {
        return None;
    };
    if costs.material_changes().len() != 1 || *currency_id <= 0 || *unit_cost <= 0 {
        return None;
    }
    Some((*currency_id, unit_cost.checked_mul(levels)?))
}

pub async fn mark_first_show(
    db: &SqlitePool,
    player_id: i64,
    id: Option<i32>,
    is_sp: Option<bool>,
) -> Result<BpMarkFirstShowReply, AppError> {
    let bp_id = id
        .or_else(task_db::current_battle_pass_id)
        .ok_or(AppError::InvalidRequest)?;
    let is_sp = is_sp.unwrap_or(false);

    battle_pass::mark_first_show(db, player_id, bp_id, is_sp).await?;

    Ok(BpMarkFirstShowReply {
        id: Some(bp_id),
        is_sp: Some(is_sp),
    })
}
