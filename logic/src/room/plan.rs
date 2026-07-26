use super::*;

pub async fn room_plan_info(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
) -> Result<GetRoomPlanInfoReply, AppError> {
    refresh_current_room_plan(db, tables, player_id).await?;
    Ok(room_plan::get_room_plan_info(db, player_id).await?)
}

pub async fn room_plan_details(
    db: &SqlitePool,
    player_id: i64,
    plan_id: i32,
) -> Result<GetRoomPlanDetailsReply, AppError> {
    Ok(GetRoomPlanDetailsReply {
        info: room_plan::get_room_plan(db, player_id, plan_id).await?,
    })
}

pub async fn set_room_plan(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    plan_id: i32,
    cover_id: i32,
    name: String,
) -> Result<SetRoomPlanReply, AppError> {
    let info =
        current_room_plan_snapshot(db, tables, player_id, plan_id, cover_id, name.clone()).await?;
    room_plan::save_room_plan(db, player_id, &info).await?;

    Ok(SetRoomPlanReply {
        id: Some(plan_id),
        cover_id: Some(cover_id),
        name: Some(name),
    })
}

pub async fn set_room_plan_name(
    db: &SqlitePool,
    player_id: i64,
    plan_id: i32,
    name: String,
) -> Result<SetRoomPlanNameReply, AppError> {
    room_plan::update_room_plan_name(db, player_id, plan_id, &name).await?;
    Ok(SetRoomPlanNameReply {
        id: Some(plan_id),
        name: Some(name),
    })
}

pub async fn set_room_plan_cover(
    db: &SqlitePool,
    player_id: i64,
    plan_id: i32,
    cover_id: i32,
) -> Result<SetRoomPlanCoverReply, AppError> {
    room_plan::update_room_plan_cover(db, player_id, plan_id, cover_id).await?;
    Ok(SetRoomPlanCoverReply {
        id: Some(plan_id),
        cover_id: Some(cover_id),
    })
}

pub async fn delete_room_plan(
    db: &SqlitePool,
    player_id: i64,
    plan_id: i32,
) -> Result<sonettobuf::DeleteRoomPlanReply, AppError> {
    room_plan::delete_room_plan(db, player_id, plan_id).await?;
    Ok(sonettobuf::DeleteRoomPlanReply { id: Some(plan_id) })
}

pub async fn switch_room_plan(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    id_a: i32,
    id_b: i32,
) -> Result<SwitchRoomPlanReply, AppError> {
    if id_a != 0 && id_b != 0 {
        room_plan::switch_room_plans(db, player_id, id_a, id_b).await?;
    } else {
        let saved_id = match (id_a, id_b) {
            (0, id) | (id, 0) if id != 0 => id,
            _ => return Err(AppError::InvalidRequest),
        };
        let current = load_current_room_plan(db, tables, player_id).await?;
        let mut selected = room_plan::get_room_plan(db, player_id, saved_id)
            .await?
            .ok_or(AppError::InvalidRequest)?;

        let mut previous = current;
        previous.id = Some(saved_id);
        selected.id = Some(0);
        room_plan::switch_active_room_plan(db, player_id, &previous, &mut selected).await?;
    }
    Ok(SwitchRoomPlanReply {
        infos: room_plan::get_room_plan_info(db, player_id).await?.infos,
    })
}

pub async fn use_room_plan(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    plan_id: i32,
) -> Result<UseRoomPlanReply, AppError> {
    switch_room_plan(db, tables, player_id, 0, plan_id).await?;
    Ok(UseRoomPlanReply { id: Some(plan_id) })
}

pub async fn share_room_plan(
    db: &SqlitePool,
    player_id: i64,
    plan_id: i32,
) -> Result<ShareRoomPlanReply, AppError> {
    let share_code = format!("{player_id}-{plan_id}");
    let can_share_count = room_plan::share_room_plan(db, player_id, plan_id, &share_code)
        .await?
        .ok_or(AppError::InvalidRequest)?;
    Ok(ShareRoomPlanReply {
        id: Some(plan_id),
        share_code: Some(share_code),
        can_share_count: Some(can_share_count),
    })
}

pub async fn get_room_share(
    db: &SqlitePool,
    share_code: String,
) -> Result<GetRoomShareReply, AppError> {
    let Some((share_user_id, info)) = room_plan::get_room_share(db, &share_code).await? else {
        return Err(AppError::InvalidRequest);
    };
    let owner = player_infos::get_player_info_data(db, share_user_id)
        .await?
        .ok_or(AppError::InvalidRequest)?;
    let road_infos = if info.id == Some(0) {
        block_packages::get_roads(db, share_user_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect()
    } else {
        Vec::new()
    };

    Ok(GetRoomShareReply {
        zone_id: Some(0),
        share_code: Some(share_code),
        room_plan_name: info.name,
        nick_name: Some(owner.user_info.username),
        share_user_id: Some(share_user_id),
        infos: info.infos,
        building_infos: info.building_infos,
        building_degree: info.building_degree,
        block_count: info.block_count,
        use_count: info.use_count,
        portrait: Some(owner.player_info.portrait),
        skins: info.skins,
        road_infos,
        change_color_count: Some(0),
    })
}

pub async fn use_room_share(
    db: &SqlitePool,
    player_id: i64,
    share_code: String,
    plan_id: i32,
    cover_id: i32,
    name: String,
) -> Result<UseRoomShareReply, AppError> {
    let Some((share_user_id, mut info)) = room_plan::get_room_share(db, &share_code).await? else {
        return Err(AppError::InvalidRequest);
    };
    let source_plan_id = info.id.unwrap_or_default();
    prepare_copied_plan(&mut info, plan_id, cover_id, &name);
    let can_use_share_count =
        room_plan::save_copied_room_plan(db, player_id, share_user_id, source_plan_id, &mut info)
            .await?
            .ok_or(AppError::InvalidRequest)?;

    Ok(UseRoomShareReply {
        share_code: Some(share_code),
        id: Some(plan_id),
        cover_id: Some(cover_id),
        name: Some(name),
        can_use_share_count: Some(can_use_share_count),
    })
}

pub async fn copy_other_room_plan(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    target_uid: i64,
    plan_id: i32,
    cover_id: i32,
    name: String,
) -> Result<CopyOtherRoomPlanReply, AppError> {
    if player_id == target_uid {
        return Err(AppError::InvalidRequest);
    }
    if plan_id == 0 {
        refresh_current_room_plan(db, tables, target_uid).await?;
    }
    let mut info = room_plan::get_room_plan(db, target_uid, plan_id)
        .await?
        .ok_or(AppError::InvalidRequest)?;
    prepare_copied_plan(&mut info, plan_id, cover_id, &name);
    room_plan::save_room_plan_with_layout(db, player_id, &mut info).await?;

    Ok(CopyOtherRoomPlanReply {
        target_uid: Some(target_uid),
        id: Some(plan_id),
        cover_id: Some(cover_id),
        name: Some(name),
    })
}

fn prepare_copied_plan(info: &mut RoomPlanInfo, plan_id: i32, cover_id: i32, name: &str) {
    info.id = Some(plan_id);
    info.cover_id = Some(cover_id);
    info.name = Some(name.to_owned());
    info.share_code = Some(String::new());
    info.use_count = Some(0);
}

pub async fn delete_room_share(
    db: &SqlitePool,
    player_id: i64,
    plan_id: i32,
) -> Result<sonettobuf::DeleteRoomShareReply, AppError> {
    room_plan::set_share_code(db, player_id, plan_id, "").await?;
    Ok(sonettobuf::DeleteRoomShareReply { id: Some(plan_id) })
}

pub async fn room_log(db: &SqlitePool, player_id: i64) -> Result<GetRoomLogReply, AppError> {
    Ok(GetRoomLogReply {
        infos: room_plan::get_room_logs(db, player_id).await?,
    })
}

async fn current_room_plan_snapshot(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    plan_id: i32,
    cover_id: i32,
    name: String,
) -> Result<RoomPlanInfo, AppError> {
    let infos: Vec<_> = block_packages::get_blocks(db, player_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    let building_infos: Vec<_> = buildings::get_placed_buildings(db, player_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    let skins = room_ob::get_skins(db, player_id)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    let building_degree =
        room_plan_building_degree(db, tables, player_id, &infos, &building_infos).await?;

    Ok(RoomPlanInfo {
        id: Some(plan_id),
        block_count: Some(infos.len() as i32),
        building_degree: Some(building_degree),
        infos,
        building_infos,
        cover_id: Some(cover_id),
        name: Some(name),
        share_code: Some(String::new()),
        use_count: Some(0),
        skins,
    })
}

async fn refresh_current_room_plan(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
) -> Result<RoomPlanInfo, AppError> {
    let current = load_current_room_plan(db, tables, player_id).await?;
    room_plan::save_room_plan(db, player_id, &current).await?;
    Ok(current)
}

async fn load_current_room_plan(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
) -> Result<RoomPlanInfo, AppError> {
    let stored = room_plan::get_room_plan(db, player_id, 0).await?;
    let mut current = current_room_plan_snapshot(
        db,
        tables,
        player_id,
        0,
        stored.as_ref().and_then(|plan| plan.cover_id).unwrap_or(1),
        stored
            .as_ref()
            .and_then(|plan| plan.name.clone())
            .unwrap_or_default(),
    )
    .await?;
    if let Some(stored) = stored {
        current.share_code = stored.share_code;
        current.use_count = stored.use_count;
    }
    Ok(current)
}

pub(super) async fn room_plan_building_degree(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    blocks: &[sonettobuf::BlockInfo],
    buildings: &[sonettobuf::BuildingInfo],
) -> Result<i32, AppError> {
    let mut block_degrees = BTreeMap::new();
    for package in block_packages::get_block_packages(db, player_id).await? {
        let degree = tables
            .block_package
            .get(package.block_package_id)
            .map_or(0, |config| config.block_build_degree);
        let used: Vec<i32> = serde_json::from_str(&package.used_block_ids)?;
        for block_id in used {
            block_degrees.insert(block_id, degree);
        }
    }

    let block_degree: i32 = blocks
        .iter()
        .map(|block| {
            let block_id = block.block_id.unwrap_or_default();
            block_degrees
                .get(&block_id)
                .copied()
                .or_else(|| block_packages::initial_block_build_degree(block_id))
                .unwrap_or_default()
        })
        .sum();
    let building_degree: i32 = buildings
        .iter()
        .map(|building| {
            tables
                .room_building
                .get(building.define_id.unwrap_or_default())
                .map_or(0, |config| config.build_degree)
        })
        .sum();
    Ok(block_degree + building_degree)
}
