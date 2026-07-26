use super::*;

pub async fn snapshot(db: &SqlitePool, hero: HeroData) -> Result<HeroInfo, AppError> {
    Ok(battle::engine::entity::stats::hero_info(db, hero).await?)
}

pub async fn mark_favor(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    is_favor: bool,
) -> Result<MarkHeroFavorReply, AppError> {
    UserHeroModel::new(player_id, db.clone())
        .set_favor(hero_id, is_favor)
        .await?;

    Ok(MarkHeroFavorReply {
        hero_id: Some(hero_id),
        is_favor: Some(is_favor),
    })
}

pub async fn unmark_new(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
) -> Result<UnMarkIsNewReply, AppError> {
    UserHeroModel::new(player_id, db.clone())
        .unmark_new(hero_id)
        .await?;

    Ok(UnMarkIsNewReply {
        hero_id: Some(hero_id),
    })
}

pub async fn unlock_voice(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    voice_id: i32,
) -> Result<UnlockVoiceReply, AppError> {
    if hero_id <= 0 || voice_id <= 0 {
        return Err(AppError::InvalidRequest);
    }
    UserHeroModel::new(player_id, db.clone())
        .unlock_voice(hero_id, voice_id)
        .await
        .map_err(|_| AppError::InvalidRequest)?;

    Ok(UnlockVoiceReply {
        hero_id: Some(hero_id),
        voice_id: Some(voice_id),
    })
}

pub async fn unlock_item(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    item_id: i32,
) -> Result<(ItemUnlockReply, (i32, i32)), AppError> {
    let tables = config::configs::get();
    let item = tables
        .character_data
        .iter()
        .find(|row| row.hero_id == hero_id && row.id == item_id && row.r#type == 2)
        .filter(|row| !row.unlock_rewards.is_empty())
        .ok_or(AppError::InvalidRequest)?;
    let heroes = UserHeroModel::new(player_id, db.clone());
    let hero = heroes
        .get_hero(hero_id)
        .await
        .map_err(|_| AppError::InvalidRequest)?;
    let mut condition = item.unlock_conditine.split('#');
    let condition_type = condition
        .next()
        .and_then(|value| value.parse::<i32>().ok())
        .ok_or(AppError::InvalidRequest)?;
    let required = condition
        .next()
        .and_then(|value| value.parse::<i32>().ok())
        .ok_or(AppError::InvalidRequest)?;
    let actual = match condition_type {
        1 => faith_percent(tables, hero.record.faith),
        2 => hero.record.rank,
        3 => hero.record.level,
        4 => hero.record.ex_skill_level,
        5 => hero.record.talent,
        6 => {
            player_infos::get_player_info_data(db, player_id)
                .await?
                .ok_or(AppError::InvalidRequest)?
                .player_info
                .last_episode_id
        }
        _ => return Err(AppError::InvalidRequest),
    };
    if actual < required {
        return Err(AppError::InvalidRequest);
    }

    let mut reward = item.unlock_rewards.split('#');
    if reward.next() != Some("2") {
        return Err(AppError::InvalidRequest);
    }
    let currency_id = reward
        .next()
        .and_then(|value| value.parse::<i32>().ok())
        .ok_or(AppError::InvalidRequest)?;
    let amount = reward
        .next()
        .and_then(|value| value.parse::<i32>().ok())
        .filter(|amount| *amount > 0)
        .ok_or(AppError::InvalidRequest)?;
    let limit = tables
        .currency
        .get(currency_id)
        .ok_or(AppError::InvalidRequest)?
        .max_limit;

    if !heroes
        .unlock_item_with_currency_reward(hero.record.uid, item_id, currency_id, amount, limit)
        .await?
    {
        return Err(AppError::InvalidRequest);
    }

    Ok((
        ItemUnlockReply {
            hero_id: Some(hero_id),
            item_id: Some(item_id),
        },
        (currency_id, amount),
    ))
}

fn faith_percent(tables: &config::GameDB, faith: i32) -> i32 {
    let mut accumulated = 0;
    let mut percent = 0;
    for level in tables.friendless.iter() {
        accumulated += level.friendliness;
        if faith < accumulated {
            return percent;
        }
        percent = level.percentage;
        if faith == accumulated {
            return percent;
        }
    }
    100
}

pub async fn use_skin(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    skin_id: i32,
) -> Result<UseSkinReply, AppError> {
    if !UserHeroModel::new(player_id, db.clone())
        .update_skin(hero_id, skin_id)
        .await?
    {
        return Err(AppError::InvalidRequest);
    }

    Ok(UseSkinReply {
        hero_id: Some(hero_id),
        skin_id: Some(skin_id),
    })
}

pub async fn read_red_dot(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    red_dot: i32,
) -> Result<HeroRedDotReadReply, AppError> {
    UserHeroModel::new(player_id, db.clone())
        .read_red_dot(hero_id, red_dot)
        .await?;

    Ok(HeroRedDotReadReply {
        hero_id: Some(hero_id),
        red_dot: Some(red_dot),
    })
}

pub async fn touch(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
) -> Result<HeroTouchReply, AppError> {
    let tables = config::configs::get();
    let faith_amount = tables
        .r#const
        .get(HeroConstId::TouchFaith as i32)
        .and_then(|row| row.value.parse().ok())
        .ok_or(AppError::InvalidRequest)?;
    let max_faith = tables.friendless.iter().map(|row| row.friendliness).sum();
    let hero = UserHeroModel::new(player_id, db.clone());
    let touch_count_left = hero.use_touch(hero_id, faith_amount, max_faith).await?;

    Ok(HeroTouchReply {
        touch_count_left: Some(touch_count_left.unwrap_or(0)),
        success: Some(touch_count_left.is_some()),
    })
}

pub async fn gain_battle_faith_in_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    player_id: i64,
    fight_group: &sonettobuf::FightGroup,
    amount: i32,
) -> Result<Vec<i32>, AppError> {
    let max_faith = config::configs::get()
        .friendless
        .iter()
        .map(|row| row.friendliness)
        .sum();
    let hero_uids = fight_group
        .hero_list
        .iter()
        .chain(&fight_group.sub_hero_list)
        .copied()
        .collect::<Vec<_>>();

    Ok(UserHeroModel::add_faith_by_uids_in_transaction(
        tx, player_id, &hero_uids, amount, max_faith,
    )
    .await?)
}

pub async fn default_equip(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    equip_uid: i64,
) -> Result<(HeroDefaultEquipReply, HeroInfo), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());
    if !hero.update_equipped_gear(hero_id, equip_uid).await? {
        return Err(AppError::InvalidRequest);
    }
    let updated = snapshot(db, hero.get_hero(hero_id).await?).await?;

    Ok((
        HeroDefaultEquipReply {
            hero_id: Some(hero_id),
            default_equip_uid: Some(equip_uid),
        },
        updated,
    ))
}

pub fn birthday(hero_id: i32) -> GetHeroBirthdayReply {
    GetHeroBirthdayReply {
        hero_id: Some(hero_id),
    }
}
