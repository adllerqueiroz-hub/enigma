use super::*;

pub async fn choice_hero_3123_weapon(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    main_id: i32,
    sub_id: i32,
) -> Result<(ChoiceHero3123WeaponReply, HeroInfo), AppError> {
    if !has_unique_skill(hero_id, UniqueSkillKind::Weapon) {
        return Err(AppError::InvalidRequest);
    }
    let hero = UserHeroModel::new(player_id, db.clone());
    let data = hero.get_hero(hero_id).await?;
    if (main_id != 0 || sub_id != 0)
        && !config::configs::get()
            .fight_eziozhuangbei
            .iter()
            .any(|row| {
                row.first_id == main_id
                    && row.second_id == sub_id
                    && row.skill_level == data.record.ex_skill_level
            })
    {
        return Err(AppError::InvalidRequest);
    }
    hero.update_special_equipped_gear(hero_id, format!("{main_id}#{sub_id}"))
        .await?;
    let updated = snapshot(db, hero.get_hero(hero_id).await?).await?;

    Ok((
        ChoiceHero3123WeaponReply {
            hero_id: Some(hero_id),
            main_id: Some(main_id),
            sub_id: Some(sub_id),
        },
        updated,
    ))
}

pub async fn choice_hero_3124_talent_tree(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    sub_id: i32,
    level: i32,
) -> Result<(ChoiceHero3124TalentTreeReply, HeroInfo), AppError> {
    let extra_str =
        update_hero_3124_talent_tree(db, player_id, hero_id, sub_id, level, true).await?;
    let updated = snapshot(
        db,
        UserHeroModel::new(player_id, db.clone())
            .get_hero(hero_id)
            .await?,
    )
    .await?;

    Ok((
        ChoiceHero3124TalentTreeReply {
            hero_id: Some(hero_id),
            extra_str: Some(extra_str),
        },
        updated,
    ))
}

pub async fn cancel_hero_3124_talent_tree(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    sub_id: i32,
    level: i32,
) -> Result<(CancelHero3124TalentTreeReply, HeroInfo), AppError> {
    let extra_str =
        update_hero_3124_talent_tree(db, player_id, hero_id, sub_id, level, false).await?;
    let updated = snapshot(
        db,
        UserHeroModel::new(player_id, db.clone())
            .get_hero(hero_id)
            .await?,
    )
    .await?;

    Ok((
        CancelHero3124TalentTreeReply {
            hero_id: Some(hero_id),
            extra_str: Some(extra_str),
        },
        updated,
    ))
}

pub async fn reset_hero_3124_talent_tree(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
) -> Result<(ResetHero3124TalentTreeReply, HeroInfo), AppError> {
    if !has_unique_skill(hero_id, UniqueSkillKind::TalentTree) {
        return Err(AppError::InvalidRequest);
    }
    let hero = UserHeroModel::new(player_id, db.clone());
    hero.update_special_equipped_gear(hero_id, String::new())
        .await?;
    let updated = snapshot(db, hero.get_hero(hero_id).await?).await?;

    Ok((
        ResetHero3124TalentTreeReply {
            hero_id: Some(hero_id),
            extra_str: Some(String::new()),
        },
        updated,
    ))
}

async fn update_hero_3124_talent_tree(
    db: &SqlitePool,
    player_id: i64,
    hero_id: i32,
    sub_id: i32,
    level: i32,
    add: bool,
) -> Result<String, AppError> {
    if !has_unique_skill(hero_id, UniqueSkillKind::TalentTree) {
        return Err(AppError::InvalidRequest);
    }
    let talent_id = hero_3124_talent_id(sub_id, level).ok_or(AppError::InvalidRequest)?;
    let hero = UserHeroModel::new(player_id, db.clone());
    let data = hero.get_hero(hero_id).await?;
    let extra_str = update_talent_extra_str(&data.record.extra_str, sub_id, level, talent_id, add);

    hero.update_special_equipped_gear(hero_id, extra_str.clone())
        .await?;

    Ok(extra_str)
}

fn has_unique_skill(hero_id: i32, kind: UniqueSkillKind) -> bool {
    config::configs::get()
        .character
        .get(hero_id)
        .and_then(|hero| hero.unique_skill_point.split_once('#'))
        .and_then(|(value, _)| value.parse::<i32>().ok())
        == Some(kind as i32)
}

pub(super) fn hero_3124_talent_id(sub_id: i32, level: i32) -> Option<i32> {
    config::configs::get()
        .hero3124_skill_talent
        .iter()
        .find(|talent| talent.sub == sub_id && talent.level == level)
        .map(|talent| talent.talent_id)
}

fn hero_3124_talent_level(sub_id: i32, talent_id: i32) -> Option<i32> {
    config::configs::get()
        .hero3124_skill_talent
        .iter()
        .find(|talent| talent.sub == sub_id && talent.talent_id == talent_id)
        .map(|talent| talent.level)
}

pub(super) fn update_talent_extra_str(
    extra_str: &str,
    sub_id: i32,
    level: i32,
    talent_id: i32,
    add: bool,
) -> String {
    let mut talents = parse_talent_extra_str(extra_str);
    let sub_talents = talents.entry(sub_id).or_default();

    if add {
        sub_talents.insert(talent_id);
    } else {
        sub_talents.retain(|id| {
            hero_3124_talent_level(sub_id, *id).is_none_or(|talent_level| talent_level < level)
        });
    }

    if sub_talents.is_empty() {
        talents.remove(&sub_id);
    }

    format_talent_extra_str(&talents)
}

fn parse_talent_extra_str(extra_str: &str) -> BTreeMap<i32, BTreeSet<i32>> {
    let mut talents = BTreeMap::new();
    for group in extra_str.split('|').filter(|group| !group.is_empty()) {
        let Some((sub_id, ids)) = group.split_once('#') else {
            continue;
        };
        let Ok(sub_id) = sub_id.parse::<i32>() else {
            continue;
        };

        talents.insert(
            sub_id,
            ids.split(',')
                .filter_map(|id| id.parse::<i32>().ok())
                .collect(),
        );
    }

    talents
}

fn format_talent_extra_str(talents: &BTreeMap<i32, BTreeSet<i32>>) -> String {
    talents
        .iter()
        .filter(|(_, ids)| !ids.is_empty())
        .map(|(sub_id, ids)| {
            let ids = ids.iter().map(i32::to_string).collect::<Vec<_>>().join(",");
            format!("{sub_id}#{ids}")
        })
        .collect::<Vec<_>>()
        .join("|")
}
