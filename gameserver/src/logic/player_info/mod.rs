use crate::{error::AppError, logic::reward};
use chrono::{Datelike, NaiveDate};
use database::db::{
    game::{items, open_infos, player_card, player_infos},
    user::account,
};
use sonettobuf::{
    GetOpenInfoReply, GetOtherPlayerInfoReply, GetPlayerInfoReply, HeroBirthdayInfo,
    HeroInfoListReply, PlayerInfoPush, RenameReply, SetBirthdayReply, SetCharacterAgeReply,
    SetPlayerBgReply, SetPortraitReply, SetShowHeroUniqueIdsReply, SetSignatureReply,
};
use sqlx::SqlitePool;
use std::collections::HashSet;

const PLAYER_SIGNATURE_LIMIT_CONST_ID: i32 = 9;
const BIRTHDAY_START_YEAR_CONST_ID: i32 = 5000;

pub async fn get_player_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetPlayerInfoReply, AppError> {
    let player_info = player_infos::get_player_info_data(db, player_id)
        .await?
        .ok_or_else(|| AppError::Custom("Player info not found".to_string()))?;

    let openinfos = open_infos::list_open_infos(db, player_id).await?;

    Ok(GetPlayerInfoReply {
        player_info: Some(player_info.into()),
        openinfos,
        can_rename: Some(true),
        main_thumbnail: Some(false),
        ext_rename: Some(0),
    })
}

pub async fn get_other_player_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetOtherPlayerInfoReply, AppError> {
    let player_info = player_infos::get_player_info_data(db, player_id)
        .await?
        .ok_or(AppError::InvalidRequest)?;
    let card = player_card::get_player_card_info(db, player_id).await?;

    Ok(GetOtherPlayerInfoReply {
        player_info: Some(player_info.into()),
        hero_cover: Some(card.hero_cover),
    })
}

pub async fn get_open_info(
    db: &SqlitePool,
    player_id: i64,
    id: i32,
) -> Result<GetOpenInfoReply, AppError> {
    Ok(GetOpenInfoReply {
        open_ifo: open_infos::get_open_info(db, player_id, id).await?,
    })
}

pub async fn hero_info_list(
    db: &SqlitePool,
    player_id: i64,
) -> Result<HeroInfoListReply, AppError> {
    let hero = database::models::game::heros::UserHeroModel::new(player_id, db.clone());
    let heroes = hero.get_all_heroes().await?;
    let touch_count = hero.get_touch_count().await?;
    let all_skins = hero.get_skins().await?;
    let birthday_infos = hero.get_birthdays().await?;

    let mut hero_infos = Vec::with_capacity(heroes.len());
    for hero in heroes {
        hero_infos.push(super::hero::snapshot(db, hero).await?);
    }

    Ok(HeroInfoListReply {
        heros: hero_infos,
        touch_count_left: touch_count,
        all_hero_skin: all_skins,
        birthday_infos: birthday_infos
            .into_iter()
            .map(|(hero_id, count)| HeroBirthdayInfo {
                hero_id: Some(hero_id),
                birthday_count: Some(count),
            })
            .collect(),
    })
}

pub async fn set_portrait(
    db: &SqlitePool,
    player_id: i64,
    portrait: i32,
) -> Result<SetPortraitReply, AppError> {
    player_infos::set_portrait(db, player_id, portrait).await?;
    Ok(SetPortraitReply {})
}

pub async fn set_signature(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    signature: String,
) -> Result<SetSignatureReply, AppError> {
    let limit = tables
        .r#const
        .get(PLAYER_SIGNATURE_LIMIT_CONST_ID)
        .and_then(|value| value.value.parse().ok())
        .ok_or(AppError::InvalidRequest)?;
    if signature.chars().count() > limit {
        return Err(AppError::InvalidRequest);
    }
    player_infos::set_signature(db, player_id, &signature).await?;
    Ok(SetSignatureReply {})
}

pub async fn set_birthday(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    birthday: String,
) -> Result<SetBirthdayReply, AppError> {
    let start_year = tables
        .r#const
        .get(BIRTHDAY_START_YEAR_CONST_ID)
        .and_then(|value| value.value.parse().ok())
        .ok_or(AppError::InvalidRequest)?;
    let date =
        NaiveDate::parse_from_str(&birthday, "%Y-%m-%d").map_err(|_| AppError::InvalidRequest)?;
    if date.year() < start_year || date > common::time::ServerTime::server_date().date_naive() {
        return Err(AppError::InvalidRequest);
    }
    if !player_infos::set_birthday_once(db, player_id, &birthday).await? {
        return Err(AppError::InvalidRequest);
    }
    Ok(SetBirthdayReply {})
}

pub async fn set_character_age(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    character_age: Vec<i32>,
) -> Result<SetCharacterAgeReply, AppError> {
    let unique = character_age.iter().copied().collect::<HashSet<_>>();
    if unique.len() != character_age.len()
        || character_age
            .iter()
            .any(|id| tables.handbook_character_age.get(*id).is_none())
    {
        return Err(AppError::InvalidRequest);
    }

    player_infos::set_character_age(db, player_id, &character_age).await?;
    Ok(SetCharacterAgeReply { character_age })
}

pub async fn set_player_bg(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    bg_id: i32,
) -> Result<SetPlayerBgReply, AppError> {
    let bg = tables
        .player_bg
        .iter()
        .find(|background| background.item == bg_id)
        .ok_or(AppError::InvalidRequest)?;
    if bg.item != 0
        && items::get_item(db, player_id, bg.item as u32)
            .await?
            .is_none_or(|item| item.quantity <= 0)
    {
        return Err(AppError::InvalidRequest);
    }
    player_infos::set_player_bg(db, player_id, bg_id).await?;
    Ok(SetPlayerBgReply { bg_id: Some(bg_id) })
}

pub async fn set_show_hero_unique_ids(
    db: &SqlitePool,
    player_id: i64,
    hero_uids: Vec<i64>,
) -> Result<(SetShowHeroUniqueIdsReply, PlayerInfoPush), AppError> {
    player_infos::set_show_hero(db, player_id, hero_uids).await?;

    Ok((SetShowHeroUniqueIdsReply {}, snapshot(db, player_id).await?))
}

pub async fn snapshot(db: &SqlitePool, player_id: i64) -> Result<PlayerInfoPush, AppError> {
    let player_info = player_infos::get_player_info_data(db, player_id)
        .await?
        .ok_or_else(|| AppError::Custom("Player info not found".to_string()))?;

    Ok(PlayerInfoPush {
        player_info: Some(player_info.into()),
    })
}

pub fn level_up_rewards(change: player_infos::PlayerLevelChange) -> reward::RewardSet {
    const POWER_CURRENCY_ID: i32 = 4;

    let mut rewards = reward::RewardSet::default();
    for level in config::configs::get()
        .player_level
        .iter()
        .filter(|level| (change.from..change.to).contains(&level.level))
    {
        rewards.extend(reward::parse_bonus(level.bonus));
        rewards
            .currencies
            .push((POWER_CURRENCY_ID, level.add_up_recover_power));
    }
    rewards
}

pub async fn rename(
    db: &SqlitePool,
    player_id: i64,
    name: String,
    guide_id: i32,
    step_id: i32,
) -> Result<(RenameReply, PlayerInfoPush), AppError> {
    account::rename_user_and_update_guide(db, player_id, name, guide_id, step_id).await?;

    Ok((
        RenameReply {
            can_rename: Some(true),
            ext_rename: Some(1),
        },
        snapshot(db, player_id).await?,
    ))
}

#[cfg(test)]
mod test;
