use super::*;

pub async fn set_room_skin(
    db: &SqlitePool,
    player_id: i64,
    part_id: i32,
    skin_id: i32,
) -> Result<SetRoomSkinReply, AppError> {
    Ok(SetRoomSkinReply {
        skin: Some(
            room_ob::set_skin(db, player_id, part_id, skin_id)
                .await?
                .into(),
        ),
    })
}

pub fn read_room_skin(skin_id: i32) -> ReadRoomSkinReply {
    ReadRoomSkinReply {
        skin_id: Some(skin_id),
    }
}

pub async fn update_room_hero_data(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    hero_ids: &[i32],
) -> Result<UpdateRoomHeroDataReply, AppError> {
    let selected = hero_ids.iter().copied().collect::<BTreeSet<_>>();
    let owned = UserHeroModel::new(player_id, db.clone())
        .get_all_heroes()
        .await?
        .into_iter()
        .map(|hero| hero.record.hero_id)
        .collect::<BTreeSet<_>>();
    if selected.len() != hero_ids.len()
        || !selected.is_subset(&owned)
        || selected.len() > room_character_limit(db, tables, player_id).await? as usize
    {
        return Err(AppError::InvalidRequest);
    }

    Ok(UpdateRoomHeroDataReply {
        room_hero_datas: room_ob::replace_heroes(db, player_id, hero_ids)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    })
}

pub async fn gain_room_hero_faith(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    hero_ids: &[i32],
) -> Result<RoomFaithGain, AppError> {
    let selected = hero_ids.iter().copied().collect::<BTreeSet<_>>();
    let placed = room_ob::get_heroes(db, player_id, &[])
        .await?
        .into_iter()
        .map(|hero| hero.hero_id)
        .collect::<BTreeSet<_>>();
    if selected.len() != hero_ids.len() || !selected.is_subset(&placed) {
        return Err(AppError::InvalidRequest);
    }

    let max_faith = tables.max_faith();
    let (room_heroes, changes) =
        room_ob::gain_hero_faith(db, player_id, hero_ids, max_faith).await?;

    Ok(RoomFaithGain {
        reply: GainRoomHeroFaithReply {
            room_hero_datas: room_heroes.into_iter().map(Into::into).collect(),
        },
        changed_hero_ids: changes.iter().map(|(hero_id, _)| *hero_id).collect(),
        material_changes: changes
            .into_iter()
            .map(|(hero_id, amount)| {
                (
                    reward::RewardMaterialType::Faith.id(),
                    hero_id as u32,
                    amount,
                )
            })
            .collect(),
    })
}

async fn room_character_limit(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
) -> Result<i32, AppError> {
    let room_level = block_packages::get_room_state(db, player_id)
        .await?
        .room_level;
    let base = tables
        .room_level(room_level)
        .map_or(0, |row| row.character_limit);
    let degree = current_room_building_degree(db, tables, player_id).await?;
    let bonus = tables
        .building_bonus(degree)
        .map_or(0, |row| row.character_limit_add);

    Ok(base + bonus)
}

pub(super) async fn current_room_building_degree(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
) -> Result<i32, AppError> {
    let blocks = block_packages::get_blocks(db, player_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>();
    let buildings = buildings::get_placed_buildings(db, player_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>();
    room_plan_building_degree(db, tables, player_id, &blocks, &buildings).await
}

pub(super) async fn room_heroes(
    db: &SqlitePool,
    player_id: i64,
    hero_ids: &[i32],
) -> Result<Vec<RoomHeroData>, AppError> {
    Ok(room_ob::get_heroes(db, player_id, hero_ids)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}
