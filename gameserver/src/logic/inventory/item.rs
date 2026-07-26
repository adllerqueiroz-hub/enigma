use super::*;

pub async fn use_items(
    db: &SqlitePool,
    player_id: i64,
    entries: Vec<M2qEntry>,
    target_id: Option<u64>,
) -> Result<
    (
        UseItemReply,
        reward::AppliedRewards,
        Vec<u32>,
        Vec<(u32, u32, i32)>,
    ),
    AppError,
> {
    if let Some(target_uid) = target_id.filter(|target_id| *target_id != 0) {
        if entries.len() != 1 {
            return Err(AppError::InvalidRequest);
        }
        let entry = &entries[0];
        let item_id = entry.material_id.ok_or(AppError::InvalidRequest)?;
        let quantity = entry.quantity.unwrap_or(1);
        let item = config::configs::get()
            .item
            .get(item_id as i32)
            .ok_or(AppError::InvalidRequest)?;
        if item.sub_type == ItemSubType::EquipmentLevelUp as i32 {
            if quantity != 1 {
                return Err(AppError::InvalidRequest);
            }
            let target_uid = i64::try_from(target_uid).map_err(|_| AppError::InvalidRequest)?;
            use_equipment_level_item(db, player_id, item_id, target_uid, &item.effect).await?;
            let changed = reward::AppliedRewards {
                equip_uids: vec![target_uid],
                ..Default::default()
            };
            return Ok((
                UseItemReply {
                    entry: entries,
                    target_id,
                },
                changed,
                vec![item_id],
                Vec::new(),
            ));
        }
    }

    let mut material_changes = Vec::new();
    let mut costs = reward::RewardSet::default();
    let mut rewards = reward::RewardSet::default();

    for entry in &entries {
        let item_id = entry.material_id.ok_or(AppError::InvalidRequest)?;
        let quantity = entry.quantity.unwrap_or(1).max(1);
        let item_rewards = item_rewards(item_id as i32, quantity, target_id)?;
        material_changes.extend(item_rewards.material_changes());
        costs.items.push((item_id, quantity));
        rewards.extend(item_rewards);
    }

    let mut tx = db.begin().await?;
    let consumed = reward::consume(&mut tx, player_id, &costs).await?;
    let changed = reward::apply_in_transaction(&mut tx, db, player_id, rewards).await?;
    tx.commit().await?;

    Ok((
        UseItemReply {
            entry: entries,
            target_id,
        },
        changed,
        consumed.item_ids,
        material_changes,
    ))
}

pub async fn use_insight_item(
    db: &SqlitePool,
    player_id: i64,
    uid: i64,
    hero_id: i32,
) -> Result<(UseInsightItemReply, i32), AppError> {
    let item = items::get_insight_item_by_uid(db, player_id, uid)
        .await?
        .ok_or(AppError::InvalidRequest)?;
    let item_id = item.item_id;

    if item.quantity <= 0 || item.expire_time <= ServerTime::now_sec_i32() {
        return Err(AppError::InvalidRequest);
    }

    let config = config::configs::get()
        .insight_item
        .get(item_id)
        .ok_or(AppError::InvalidRequest)?;
    let target_rank = config.hero_rank + 1;
    let target_level = config
        .effect
        .split('#')
        .nth(1)
        .and_then(|level| level.parse().ok())
        .unwrap_or(1);

    let heroes = UserHeroModel::new(player_id, db.clone());
    let current = heroes.get(hero_id).await?;
    let character = config::configs::get()
        .character
        .get(hero_id)
        .ok_or(AppError::InvalidRequest)?;
    if current.record.rank >= target_rank
        || !config
            .hero_rares
            .split('#')
            .filter_map(|rare| rare.parse::<i32>().ok())
            .any(|rare| rare == character.rare + 1)
    {
        return Err(AppError::InvalidRequest);
    }
    if !heroes
        .apply_insight_item(InsightUpgrade {
            item_uid: uid,
            item_id,
            hero_id,
            current_rank: current.record.rank,
            current_level: current.record.level,
            target_rank,
            target_level,
        })
        .await?
    {
        return Err(AppError::InvalidRequest);
    }

    Ok((
        UseInsightItemReply {
            uid: Some(uid),
            hero_id: Some(hero_id),
        },
        item_id,
    ))
}

pub async fn mark_read_sub_type21(
    db: &SqlitePool,
    player_id: i64,
    item_id: i32,
) -> Result<MarkReadSubType21Reply, AppError> {
    red_dots::hide_red_dot_infos(
        db,
        player_id,
        RedDotId::PlayerChangeBgItemNew.id(),
        vec![item_id],
    )
    .await?;

    Ok(MarkReadSubType21Reply {
        item_id: Some(item_id),
    })
}

pub(super) fn item_rewards(
    item_id: i32,
    quantity: i32,
    target_id: Option<u64>,
) -> Result<reward::RewardSet, AppError> {
    let item = config::configs::get()
        .item
        .get(item_id)
        .ok_or(AppError::InvalidRequest)?;

    let mut rewards =
        if item.sub_type == ItemSubType::EquipmentLevelUp as i32 && target_id == Some(0) {
            reward::parse(&item.effect)
        } else if let Some(target_id) = target_id {
            target_item_rewards(&item.effect, target_id as i32).unwrap_or_else(|| {
                let mut selected = reward::RewardSet::default();
                selected.items.push((target_id as u32, 1));
                selected
            })
        } else {
            reward::parse(&item.effect)
        };

    for (_, amount) in rewards.items.iter_mut() {
        *amount *= quantity;
    }
    for (_, amount) in rewards.currencies.iter_mut() {
        *amount *= quantity;
    }
    for (_, amount) in rewards.block_packages.iter_mut() {
        *amount *= quantity;
    }
    for (_, amount) in rewards.heroes.iter_mut() {
        *amount *= quantity;
    }
    for (_, amount) in rewards.equips.iter_mut() {
        *amount *= quantity;
    }
    for (_, amount) in rewards.power_items.iter_mut() {
        *amount *= quantity;
    }
    for (_, amount) in rewards.room_buildings.iter_mut() {
        *amount *= quantity;
    }
    for (_, amount) in rewards.special_blocks.iter_mut() {
        *amount *= quantity;
    }
    for (_, amount) in rewards.insight_items.iter_mut() {
        *amount *= quantity;
    }

    if rewards.is_empty() {
        Err(AppError::InvalidRequest)
    } else {
        Ok(rewards)
    }
}
