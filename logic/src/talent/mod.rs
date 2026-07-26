use crate::error::AppError;
use database::models::game::heros::{HeroModel, UserHeroModel};
use sonettobuf::{
    HeroInfo, HeroTalentStylePercent, HeroTalentStyleStatReply, HeroTalentUpReply,
    PutTalentCubeBatchReply, PutTalentCubeReply, PutTalentSchemeReply, RenameTalentTemplateReply,
    TakeoffAllTalentCubeReply, TalentStyleReadReply, UnlockTalentStyleReply, UseTalentStyleReply,
    UseTalentTemplateReply,
};
use sqlx::SqlitePool;

pub async fn style_read(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
) -> Result<(TalentStyleReadReply, HeroInfo), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());
    hero.talent_style_read(hero_id).await?;
    let updated = super::hero::snapshot(db, hero.get(hero_id).await?).await?;

    Ok((
        TalentStyleReadReply {
            hero_id: Some(hero_id),
        },
        updated,
    ))
}

pub async fn talent_up(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
) -> Result<(HeroTalentUpReply, HeroInfo), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());
    let current = hero.get(hero_id).await?.record.talent;
    let next_talent = config::configs::get()
        .character_talent
        .iter()
        .filter(|row| row.hero_id == hero_id && row.talent_id > current)
        .map(|row| row.talent_id)
        .min()
        .ok_or(AppError::InvalidRequest)?;

    hero.update_talent(hero_id, next_talent).await?;
    let updated = super::hero::snapshot(db, hero.get(hero_id).await?).await?;

    Ok((
        HeroTalentUpReply {
            hero_id: Some(hero_id),
            talent_id: Some(next_talent),
        },
        updated,
    ))
}

pub async fn put_cube(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    template_id: i32,
    get_cube: Option<(i32, i32)>,
    put_cube: Option<(i32, i32, i32, i32)>,
) -> Result<(PutTalentCubeReply, HeroInfo), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());

    if let Some((pos_x, pos_y)) = get_cube {
        hero.remove_talent_cube(hero_id, template_id, pos_x, pos_y)
            .await?;
    }

    if let Some((cube_id, direction, pos_x, pos_y)) = put_cube {
        hero.place_talent_cube(hero_id, template_id, cube_id, direction, pos_x, pos_y)
            .await?;
    }

    hero.sync_active_talent_cubes(hero_id, template_id, get_cube, put_cube)
        .await?;
    let template_info = hero.get_template_info(hero_id, template_id).await?;
    let updated = super::hero::snapshot(db, hero.get(hero_id).await?).await?;

    Ok((
        PutTalentCubeReply {
            hero_id: Some(hero_id),
            template_info: Some(template_info),
        },
        updated,
    ))
}

pub async fn put_cube_batch(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    template_id: i32,
    style: Option<i32>,
    cubes: Vec<(i32, i32, i32, i32)>,
) -> Result<(PutTalentCubeBatchReply, HeroInfo), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());
    hero.replace_talent_cubes(hero_id, template_id, cubes)
        .await?;

    if let Some(style) = style {
        hero.apply_talent_style(hero_id, template_id, style).await?;
    }

    let template_info = hero.get_template_info(hero_id, template_id).await?;
    let updated = super::hero::snapshot(db, hero.get(hero_id).await?).await?;

    Ok((
        PutTalentCubeBatchReply {
            hero_id: Some(hero_id),
            style,
            template_info: Some(template_info),
        },
        updated,
    ))
}

pub async fn takeoff_all(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    template_id: i32,
) -> Result<(TakeoffAllTalentCubeReply, HeroInfo), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());
    let template_info = hero
        .replace_talent_cubes(hero_id, template_id, Vec::new())
        .await?;
    let updated = super::hero::snapshot(db, hero.get(hero_id).await?).await?;

    Ok((
        TakeoffAllTalentCubeReply {
            hero_id: Some(hero_id),
            template_info: Some(template_info),
        },
        updated,
    ))
}

pub async fn rename_template(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    template_id: i32,
    name: String,
) -> Result<RenameTalentTemplateReply, AppError> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > 10 {
        return Err(AppError::InvalidRequest);
    }
    let template_info = UserHeroModel::new(player_id, db.clone())
        .rename_talent_template(hero_id, template_id, name)
        .await
        .map_err(|_| AppError::InvalidRequest)?;

    Ok(RenameTalentTemplateReply {
        hero_id: Some(hero_id),
        template_info: Some(template_info),
    })
}

pub async fn put_scheme(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    talent_id: i32,
    talent_mould: i32,
    template_id: i32,
) -> Result<(PutTalentSchemeReply, HeroInfo), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());
    let template_info = hero
        .load_talent_scheme(hero_id, talent_id, talent_mould, template_id)
        .await?;
    let updated = super::hero::snapshot(db, hero.get(hero_id).await?).await?;

    Ok((
        PutTalentSchemeReply {
            hero_id: Some(hero_id),
            template_info: Some(template_info),
        },
        updated,
    ))
}

pub fn style_stat(hero_id: i32) -> HeroTalentStyleStatReply {
    let style_percent_list = config::configs::get()
        .talent_style_cost
        .iter()
        .filter(|row| row.hero_id == hero_id)
        .map(|row| HeroTalentStylePercent {
            style: Some(row.style_id),
            percent: Some(0),
        })
        .collect();

    HeroTalentStyleStatReply {
        hero_id: Some(hero_id),
        style_percent_list,
    }
}

pub async fn unlock_style(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    style: i32,
) -> Result<(UnlockTalentStyleReply, HeroInfo), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());
    if !hero.has_talent_style(hero_id, style).await? {
        hero.unlock_talent_style(hero_id, style).await?;
    }
    let updated = super::hero::snapshot(db, hero.get(hero_id).await?).await?;

    Ok((
        UnlockTalentStyleReply {
            hero_id: Some(hero_id),
            style: Some(style),
        },
        updated,
    ))
}

pub async fn use_style(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    template_id: i32,
    style: i32,
) -> Result<(UseTalentStyleReply, HeroInfo), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());
    hero.apply_talent_style(hero_id, template_id, style).await?;
    let updated = super::hero::snapshot(db, hero.get(hero_id).await?).await?;

    Ok((
        UseTalentStyleReply {
            hero_id: Some(hero_id),
            template_id: Some(template_id),
            style: Some(style),
        },
        updated,
    ))
}

pub async fn use_template(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    template_id: i32,
) -> Result<(UseTalentTemplateReply, HeroInfo), AppError> {
    let hero = UserHeroModel::new(player_id, db.clone());
    let template_info = hero.switch_talent_template(hero_id, template_id).await?;
    let updated = super::hero::snapshot(db, hero.get(hero_id).await?).await?;

    Ok((
        UseTalentTemplateReply {
            hero_id: Some(hero_id),
            template_info: Some(template_info),
        },
        updated,
    ))
}

#[cfg(test)]
mod test;
