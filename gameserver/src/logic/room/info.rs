use super::*;

pub async fn room_info(db: &SqlitePool, player_id: i64) -> Result<GetRoomInfoReply, AppError> {
    Ok(GetRoomInfoReply {
        infos: block_packages::get_blocks(db, player_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
        is_reset: Some(block_packages::get_room_reset_state(db, player_id).await?),
        building_infos: buildings::get_placed_buildings(db, player_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
        block_packages: block_packages::get_block_packages(db, player_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
        road_infos: block_packages::get_roads(db, player_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    })
}

pub async fn room_ob_info(
    db: &SqlitePool,
    player_id: i64,
    need_block_data: bool,
) -> Result<GetRoomObInfoReply, AppError> {
    let state = block_packages::get_room_state(db, player_id).await?;
    let committed = block_packages::committed_room_info(db, player_id).await?;

    Ok(GetRoomObInfoReply {
        infos: if need_block_data {
            if let Some(snapshot) = &committed {
                snapshot.infos.clone()
            } else {
                block_packages::get_blocks(db, player_id)
                    .await?
                    .into_iter()
                    .map(Into::into)
                    .collect()
            }
        } else {
            Vec::new()
        },
        building_infos: if let Some(snapshot) = &committed {
            snapshot.building_infos.clone()
        } else {
            buildings::get_placed_buildings(db, player_id)
                .await?
                .into_iter()
                .map(Into::into)
                .collect()
        },
        formula_infos: room_ob::get_formulas(db, player_id)
            .await?
            .into_iter()
            .flat_map(|formula| formula.into_proto())
            .collect(),
        room_level: Some(state.room_level),
        room_hero_datas: room_ob::get_heroes(db, player_id, &[])
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
        production_lines: room_ob::get_production_lines(db, player_id, &[])
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
        has_get_room_themes: state.room_theme_ids,
        need_block_data: Some(need_block_data),
        skins: room_ob::get_skins(db, player_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
        road_infos: if let Some(snapshot) = committed {
            snapshot.road_infos
        } else {
            block_packages::get_roads(db, player_id)
                .await?
                .into_iter()
                .map(Into::into)
                .collect()
        },
        have_fishing_bonus: state.have_fishing_bonus.then_some(true),
    })
}
