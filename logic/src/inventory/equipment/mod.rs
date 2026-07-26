use crate::{error::AppError, reward};
use database::{db::game::equipment, models::game::equipment::UserEquipmentModel};
use sonettobuf::{
    EatEquip, EquipBreakReply, EquipDecomposeReply, EquipLockReply, EquipRefineReply,
    EquipStrengthenReply, GetEquipInfoReply,
};
use sqlx::SqlitePool;
use std::collections::HashSet;

const DECOMPOSE_MAX_COUNT: usize = 100;

pub(super) async fn equip_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetEquipInfoReply, AppError> {
    Ok(GetEquipInfoReply {
        equips: equipment::get_user_equipment(db, player_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    })
}

pub(super) async fn equip_lock(
    db: &SqlitePool,
    player_id: i64,
    target_uid: i64,
    lock: bool,
) -> Result<EquipLockReply, AppError> {
    if !equipment::update_equipment_lock(db, player_id, target_uid, lock).await? {
        return Err(AppError::InvalidRequest);
    }

    Ok(EquipLockReply {
        target_uid: Some(target_uid),
        lock: Some(lock),
    })
}

pub(super) async fn strengthen(
    db: &SqlitePool,
    player_id: i64,
    target_uid: i64,
    eat_equips: Vec<EatEquip>,
) -> Result<(EquipStrengthenReply, Vec<i64>), AppError> {
    let Some(consumes) = eat_equips
        .iter()
        .map(|equip| Some((equip.eat_uid?, equip.count.unwrap_or(1))))
        .collect::<Option<Vec<_>>>()
    else {
        return Err(AppError::InvalidRequest);
    };
    if !valid_strengthen_consumes(&consumes) {
        return Err(AppError::InvalidRequest);
    }

    let (total_exp, _) = UserEquipmentModel::new(player_id, db.clone())
        .strengthen_equip(target_uid, consumes)
        .await?;

    Ok((
        EquipStrengthenReply {
            target_uid: Some(target_uid),
            eat_equips,
        },
        if total_exp > 0 {
            vec![target_uid]
        } else {
            Vec::new()
        },
    ))
}

fn valid_strengthen_consumes(consumes: &[(i64, i32)]) -> bool {
    !consumes.is_empty()
        && consumes.iter().all(|(_, count)| *count > 0)
        && consumes
            .iter()
            .map(|(uid, _)| *uid)
            .collect::<HashSet<_>>()
            .len()
            == consumes.len()
}

pub(super) async fn break_equip(
    db: &SqlitePool,
    player_id: i64,
    target_uid: i64,
) -> Result<(EquipBreakReply, Vec<(i32, i32)>, Vec<u32>, Vec<i64>), AppError> {
    let equips = UserEquipmentModel::new(player_id, db.clone());
    let target = equips.get_equip(target_uid).await?;
    let tables = config::configs::get();
    let equip_cfg = tables
        .equip
        .get(target.equip_id)
        .ok_or(AppError::InvalidRequest)?;
    let current = tables
        .equip_break_cost(equip_cfg.rare, target.break_lv)
        .ok_or(AppError::InvalidRequest)?;

    if target.level < current.level {
        return Ok((EquipBreakReply {}, Vec::new(), Vec::new(), Vec::new()));
    }

    let Some(next) = tables.equip_break_cost(equip_cfg.rare, target.break_lv + 1) else {
        return Ok((EquipBreakReply {}, Vec::new(), Vec::new(), Vec::new()));
    };

    let costs = reward::parse(&next.cost);
    let mut all_costs = costs.clone();
    if next.score_cost > 0 {
        all_costs.currencies.push((1, next.score_cost));
    }
    let mut tx = db.begin().await?;
    match reward::consume(&mut tx, player_id, &all_costs).await {
        Ok(_) => {}
        Err(AppError::InsufficientItems | AppError::InsufficientCurrency) => {
            return Ok((EquipBreakReply {}, Vec::new(), Vec::new(), Vec::new()));
        }
        Err(error) => return Err(error),
    }
    if !equipment::advance_break_level_in_transaction(
        &mut tx,
        player_id,
        target_uid,
        target.break_lv,
    )
    .await?
    {
        return Ok((EquipBreakReply {}, Vec::new(), Vec::new(), Vec::new()));
    }
    tx.commit().await?;

    let changed_items = costs.items.iter().map(|(id, _)| *id).collect::<Vec<_>>();
    let changed_currencies = if next.score_cost > 0 {
        vec![(1, -next.score_cost)]
    } else {
        Vec::new()
    };
    let changed_uids = vec![target_uid];

    Ok((
        EquipBreakReply {},
        changed_currencies,
        changed_items,
        changed_uids,
    ))
}

pub(super) async fn refine(
    db: &SqlitePool,
    player_id: i64,
    target_uid: i64,
    eat_uids: Vec<i64>,
) -> Result<(EquipRefineReply, Vec<i64>, Vec<i64>), AppError> {
    if eat_uids.is_empty()
        || eat_uids.iter().copied().collect::<HashSet<_>>().len() != eat_uids.len()
    {
        return Err(AppError::InvalidRequest);
    }

    let mut tx = db.begin().await?;
    if equipment::refine_equipment(&mut tx, player_id, target_uid, &eat_uids)
        .await?
        .is_none()
    {
        return Err(AppError::InvalidRequest);
    }
    tx.commit().await?;

    Ok((
        EquipRefineReply {
            target_uid: Some(target_uid),
            eat_uids: eat_uids.clone(),
        },
        vec![target_uid],
        eat_uids,
    ))
}

pub(super) async fn decompose(
    db: &SqlitePool,
    player_id: i64,
    equip_uids: Vec<i64>,
) -> Result<(EquipDecomposeReply, Vec<i64>), AppError> {
    if equip_uids.is_empty()
        || equip_uids.len() > DECOMPOSE_MAX_COUNT
        || equip_uids.iter().copied().collect::<HashSet<_>>().len() != equip_uids.len()
    {
        return Err(AppError::InvalidRequest);
    }

    let tables = config::configs::get();
    let equips = UserEquipmentModel::new(player_id, db.clone());
    let mut rarities = Vec::with_capacity(equip_uids.len());
    for uid in &equip_uids {
        let equip = equips
            .get_equip(*uid)
            .await
            .map_err(|_| AppError::InvalidRequest)?;
        let equip_cfg = tables
            .equip
            .get(equip.equip_id)
            .ok_or(AppError::InvalidRequest)?;
        if equip.is_lock
            || equip.level != 1
            || equip.count != 1
            || equip_cfg.rare >= 4
            || equip_cfg.is_exp_equip != 0
            || equip_cfg.is_sp_refine != 0
        {
            return Err(AppError::InvalidRequest);
        }
        rarities.push(equip_cfg.rare);
    }

    let base_exp = &tables
        .equip_const
        .get(2)
        .ok_or(AppError::InvalidRequest)?
        .value;
    let (output_equip_id, output_unit_count) = decompose_config(
        &tables
            .equip_const
            .get(17)
            .ok_or(AppError::InvalidRequest)?
            .value,
    )
    .ok_or(AppError::InvalidRequest)?;
    let output_count =
        decompose_count(base_exp, rarities, output_unit_count).ok_or(AppError::InvalidRequest)?;
    let changed_uids =
        equipment::decompose_equipment(db, player_id, &equip_uids, output_equip_id, output_count)
            .await?;

    Ok((EquipDecomposeReply { equip_uids }, changed_uids))
}

fn decompose_count(
    base_exp: &str,
    rarities: impl IntoIterator<Item = i32>,
    unit_count: i32,
) -> Option<i32> {
    let exp = rarities.into_iter().try_fold(0i32, |total, rarity| {
        total.checked_add(config_pair(base_exp, rarity)?)
    })?;
    (exp / 100)
        .checked_mul(unit_count)
        .filter(|count| *count > 0)
}

fn config_pair(value: &str, key: i32) -> Option<i32> {
    value.split('|').find_map(|entry| {
        let (entry_key, entry_value) = entry.split_once('#')?;
        if entry_key.parse::<i32>().ok()? == key {
            entry_value.parse().ok()
        } else {
            None
        }
    })
}

fn decompose_config(value: &str) -> Option<(i32, i32)> {
    let mut values = value.split('#').skip(1);
    Some((values.next()?.parse().ok()?, values.next()?.parse().ok()?))
}

#[cfg(test)]
mod test;
