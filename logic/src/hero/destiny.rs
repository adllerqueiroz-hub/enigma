use super::*;

pub async fn destiny_stone(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    stone_id: i32,
) -> Result<(DestinyStoneUseReply, HeroInfo), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());
    let current = hero.get(hero_id).await?;
    if stone_id != 0
        && (!destiny_stones(hero_id).contains(&stone_id)
            || !current.destiny_stone_unlocks.contains(&stone_id))
    {
        return Err(AppError::InvalidRequest);
    }
    hero.update_destiny_stone(hero_id, stone_id).await?;
    let updated = snapshot(db, hero.get(hero_id).await?).await?;

    Ok((
        DestinyStoneUseReply {
            hero_id: Some(hero_id),
            stone_id: Some(stone_id),
        },
        updated,
    ))
}

pub async fn destiny_rank_up(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
) -> Result<(DestinyRankUpReply, HeroInfo, ConsumedRewards), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());
    let current = hero.get(hero_id).await?;
    let slot = next_destiny_slot(
        hero_id,
        current.record.destiny_rank,
        current.record.destiny_level,
    )
    .filter(|slot| slot.node == 1 && slot.stage == current.record.destiny_rank.saturating_add(1))
    .ok_or(AppError::InvalidRequest)?;
    let mut tx = db.begin().await?;
    let consumed = reward::consume(&mut tx, player_id, &reward::parse(&slot.consume)).await?;
    if !hero
        .update_destiny_progress_in_transaction(
            &mut tx,
            hero_id,
            current.record.destiny_rank,
            current.record.destiny_level,
            slot.stage,
            slot.node,
        )
        .await?
    {
        return Err(AppError::InvalidRequest);
    }
    tx.commit().await?;
    let updated = snapshot(db, hero.get(hero_id).await?).await?;

    Ok((
        DestinyRankUpReply {
            hero_id: Some(hero_id),
        },
        updated,
        consumed,
    ))
}

pub async fn destiny_level_up(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    level: i32,
) -> Result<(DestinyLevelUpReply, HeroInfo, ConsumedRewards), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());
    let current = hero.get(hero_id).await?;
    if current.record.destiny_rank <= 0 || level <= current.record.destiny_level {
        return Err(AppError::InvalidRequest);
    }

    let destiny = config::configs::get()
        .character_destiny
        .iter()
        .find(|row| row.hero_id == hero_id)
        .ok_or(AppError::InvalidRequest)?;
    let slots = (current.record.destiny_level + 1..=level)
        .map(|node| {
            config::configs::get()
                .character_destiny_slots
                .iter()
                .find(|slot| {
                    slot.slots_id == destiny.slots_id
                        && slot.stage == current.record.destiny_rank
                        && slot.node == node
                })
                .ok_or(AppError::InvalidRequest)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let costs = slots
        .iter()
        .map(|slot| slot.consume.as_str())
        .filter(|cost| !cost.is_empty())
        .collect::<Vec<_>>()
        .join("|");
    let mut tx = db.begin().await?;
    let consumed = reward::consume(&mut tx, player_id, &reward::parse(&costs)).await?;
    if !hero
        .update_destiny_progress_in_transaction(
            &mut tx,
            hero_id,
            current.record.destiny_rank,
            current.record.destiny_level,
            current.record.destiny_rank,
            level,
        )
        .await?
    {
        return Err(AppError::InvalidRequest);
    }
    tx.commit().await?;
    let updated = snapshot(db, hero.get(hero_id).await?).await?;

    Ok((
        DestinyLevelUpReply {
            hero_id: Some(hero_id),
            level: Some(level),
        },
        updated,
        consumed,
    ))
}

pub async fn destiny_stone_unlock(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    stone_id: i32,
) -> Result<(DestinyStoneUnlockReply, HeroInfo, ConsumedRewards), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());
    let current = hero.get(hero_id).await?;
    if current.record.destiny_rank <= 0
        || !destiny_stones(hero_id).contains(&stone_id)
        || current.destiny_stone_unlocks.contains(&stone_id)
    {
        return Err(AppError::InvalidRequest);
    }
    let config = config::configs::get()
        .character_destiny_facets_consume
        .iter()
        .find(|row| row.facets_id == stone_id)
        .ok_or(AppError::InvalidRequest)?;
    let mut tx = db.begin().await?;
    let consumed = reward::consume(&mut tx, player_id, &reward::parse(&config.consume)).await?;
    if !hero
        .unlock_destiny_stone_in_transaction(&mut tx, hero_id, stone_id)
        .await?
    {
        return Err(AppError::InvalidRequest);
    }
    tx.commit().await?;
    let updated = snapshot(db, hero.get(hero_id).await?).await?;

    Ok((
        DestinyStoneUnlockReply {
            hero_id: Some(hero_id),
            stone_id: Some(stone_id),
        },
        updated,
        consumed,
    ))
}

pub(super) fn destiny_stones(hero_id: i32) -> Vec<i32> {
    config::configs::get()
        .character_destiny
        .iter()
        .find(|row| row.hero_id == hero_id)
        .map(|row| {
            row.facets_id
                .split('#')
                .filter_map(|id| id.parse().ok())
                .collect()
        })
        .unwrap_or_default()
}

pub(super) fn next_destiny_slot(
    hero_id: i32,
    rank: i32,
    level: i32,
) -> Option<&'static config::character_destiny_slots::CharacterDestinySlots> {
    let tables = config::configs::get();
    let slots_id = tables
        .character_destiny
        .iter()
        .find(|row| row.hero_id == hero_id)?
        .slots_id;
    let find = |stage, node| {
        tables
            .character_destiny_slots
            .iter()
            .find(|slot| slot.slots_id == slots_id && slot.stage == stage && slot.node == node)
    };

    if rank == 0 {
        find(1, 1)
    } else {
        find(rank, level + 1).or_else(|| find(rank + 1, 1))
    }
}
