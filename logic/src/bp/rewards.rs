use super::*;

pub async fn get_bp_bonus(
    db: &SqlitePool,
    player_id: i64,
    id: Option<i32>,
    level: Option<i32>,
    pay_bonus: Option<bool>,
    is_sp: Option<bool>,
) -> Result<BpBonusClaim, AppError> {
    let bp_id = id
        .or_else(task_db::current_battle_pass_id)
        .ok_or(AppError::InvalidRequest)?;
    let bp = config::configs::get()
        .battle_pass(bp_id)
        .ok_or(AppError::InvalidRequest)?;
    let state = battle_pass::get_state(db, player_id, bp_id).await?;
    let owned_skins = database::models::game::heros::UserHeroModel::new(player_id, db.clone())
        .get_skins()
        .await?;
    let max_claim_level = state.score / bp.exp_level_up.max(1);
    let claim_all = level.unwrap_or_default() <= 0;
    let wants_sp = is_sp.unwrap_or(false);
    let wants_pay = pay_bonus.unwrap_or(false);
    let include_free = !wants_sp && (!wants_pay || claim_all);
    let include_pay = !wants_sp && (wants_pay || claim_all) && state.pay_status > 0;
    let include_sp_free = wants_sp && (!wants_pay || claim_all);
    let include_sp_pay = wants_sp && (wants_pay || claim_all) && state.pay_status > 0;

    let mut rewards = reward::RewardSet::default();
    let mut free_levels = Vec::new();
    let mut pay_levels = Vec::new();
    let mut sp_free_levels = Vec::new();
    let mut sp_pay_levels = Vec::new();

    for bonus in config::configs::get()
        .battle_pass_bonuses(bp_id)
        .filter(|bonus| bonus.level <= max_claim_level && (claim_all || Some(bonus.level) == level))
    {
        if include_free
            && !bonus.free_bonus.is_empty()
            && !state.has_get_free_bonus.contains(&bonus.level)
        {
            rewards.extend(parse_bp_reward(&bonus.free_bonus, &owned_skins));
            free_levels.push(bonus.level);
        }
        if include_pay
            && !bonus.pay_bonus.is_empty()
            && !state.has_get_pay_bonus.contains(&bonus.level)
        {
            rewards.extend(parse_bp_reward(&bonus.pay_bonus, &owned_skins));
            pay_levels.push(bonus.level);
        }
        if include_sp_free
            && !bonus.sp_free_bonus.is_empty()
            && !state.has_get_sp_free_bonus.contains(&bonus.level)
        {
            rewards.extend(parse_bp_reward(&bonus.sp_free_bonus, &owned_skins));
            sp_free_levels.push(bonus.level);
        }
        if include_sp_pay
            && !bonus.sp_pay_bonus.is_empty()
            && !state.has_get_sp_pay_bonus.contains(&bonus.level)
        {
            rewards.extend(parse_bp_reward(&bonus.sp_pay_bonus, &owned_skins));
            sp_pay_levels.push(bonus.level);
        }
    }

    let material_changes = rewards.material_changes();
    let mut tx = db.begin().await?;
    let state = battle_pass::claim_bonus_levels_in_transaction(
        &mut tx,
        player_id,
        bp_id,
        &state,
        battle_pass::BattlePassClaimLevels {
            free: &free_levels,
            pay: &pay_levels,
            sp_free: &sp_free_levels,
            sp_pay: &sp_pay_levels,
        },
    )
    .await?
    .ok_or(AppError::InvalidRequest)?;
    let applied_rewards = reward::apply_in_transaction(&mut tx, db, player_id, rewards).await?;
    tx.commit().await?;
    let mut infos = score_bonus_info(bp_id, Some(&state));

    if !claim_all {
        infos.retain(|info| info.level == level);
    }

    Ok(BpBonusClaim {
        reply: GetBpBonusReply {
            id: Some(bp_id),
            score_bonus_info: infos,
        },
        rewards: applied_rewards,
        material_changes,
    })
}

pub async fn get_self_select_bonus(
    db: &SqlitePool,
    player_id: i64,
    id: Option<i32>,
    level: Option<i32>,
    index: Option<i32>,
) -> Result<BpSelfSelectClaim, AppError> {
    let bp_id = id
        .or_else(task_db::current_battle_pass_id)
        .ok_or(AppError::InvalidRequest)?;
    let level = level.ok_or(AppError::InvalidRequest)?;
    let index = index.ok_or(AppError::InvalidRequest)?;
    let tables = config::configs::get();
    let bp = tables.battle_pass(bp_id).ok_or(AppError::InvalidRequest)?;
    let bonus = tables
        .battle_pass_bonus(bp_id, level)
        .ok_or(AppError::InvalidRequest)?;
    let selected =
        select_reward(&bonus.self_select_pay_bonus, index).ok_or(AppError::InvalidRequest)?;
    let state = battle_pass::get_state(db, player_id, bp_id).await?;
    if state.pay_status == 0
        || state.score / bp.exp_level_up.max(1) < level
        || state
            .has_get_self_select_bonus
            .iter()
            .any(|bonus| bonus.level == Some(level))
    {
        return Err(AppError::InvalidRequest);
    }

    let owned_skins = database::models::game::heros::UserHeroModel::new(player_id, db.clone())
        .get_skins()
        .await?;
    let rewards = parse_bp_reward(selected, &owned_skins);
    if rewards.is_empty() {
        return Err(AppError::InvalidRequest);
    }
    let material_changes = rewards.material_changes();
    let mut tx = db.begin().await?;
    battle_pass::claim_self_select_bonus_in_transaction(
        &mut tx, player_id, bp_id, &state, level, index,
    )
    .await?
    .ok_or(AppError::InvalidRequest)?;
    let rewards = reward::apply_in_transaction(&mut tx, db, player_id, rewards).await?;
    tx.commit().await?;

    Ok(BpSelfSelectClaim {
        reply: GetSelfSelectBonusReply {
            id: Some(bp_id),
            level: Some(level),
            index: Some(index),
        },
        rewards,
        material_changes,
    })
}

pub(super) fn select_reward(value: &str, index: i32) -> Option<&str> {
    value.split('|').nth(usize::try_from(index).ok()?)
}
