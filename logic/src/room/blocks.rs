use super::*;

impl RoomManager {
    pub async fn block_package_info(
        &self,
        db: &SqlitePool,
    ) -> Result<GetBlockPackageInfoReply, AppError> {
        Ok(GetBlockPackageInfoReply {
            block_package_ids: block_packages::get_block_packages(db, self.player_id)
                .await?
                .into_iter()
                .map(|package| package.block_package_id)
                .collect(),
            special_blocks: block_packages::get_special_blocks(db, self.player_id)
                .await?
                .into_iter()
                .map(Into::into)
                .collect(),
        })
    }

    pub async fn hide_block_package_reddot(
        &self,
        db: &SqlitePool,
        package_id: i32,
    ) -> Result<HideBlockPackageReddotReply, AppError> {
        if !block_packages::get_block_packages(db, self.player_id)
            .await?
            .iter()
            .any(|package| package.block_package_id == package_id)
        {
            return Err(AppError::InvalidRequest);
        }
        red_dots::hide_red_dot_infos(
            db,
            self.player_id,
            RedDotId::RoomBlockPackage.id(),
            vec![package_id],
        )
        .await?;
        Ok(HideBlockPackageReddotReply {
            id: Some(package_id),
        })
    }

    pub async fn hide_building_reddot(
        &self,
        db: &SqlitePool,
        building_id: i32,
    ) -> Result<HideBuildingReddotReply, AppError> {
        if !buildings::get_user_buildings(db, self.player_id)
            .await?
            .iter()
            .any(|building| building.define_id == building_id)
        {
            return Err(AppError::InvalidRequest);
        }
        red_dots::hide_red_dot_infos(
            db,
            self.player_id,
            RedDotId::RoomBuildingPlace.id(),
            vec![building_id],
        )
        .await?;
        Ok(HideBuildingReddotReply {
            id: Some(building_id),
        })
    }

    pub async fn block_permanent_info(
        &self,
        db: &SqlitePool,
        tables: &config::GameDB,
        requested_block_ids: Vec<i32>,
    ) -> Result<GetBlockPermanentInfoReply, AppError> {
        let mut owned = BTreeSet::new();

        for package in block_packages::get_block_packages(db, self.player_id).await? {
            owned.extend(
                serde_json::from_str::<Vec<i32>>(&package.unused_block_ids).unwrap_or_default(),
            );
            owned.extend(
                serde_json::from_str::<Vec<i32>>(&package.used_block_ids).unwrap_or_default(),
            );
        }

        for block in block_packages::get_special_blocks(db, self.player_id).await? {
            owned.insert(block.block_id);
        }

        for block in block_packages::get_blocks(db, self.player_id).await? {
            owned.insert(block.block_id);
        }

        let block_ids: BTreeSet<_> = if requested_block_ids.is_empty() {
            owned
        } else {
            requested_block_ids
                .into_iter()
                .filter(|id| owned.contains(id))
                .collect()
        };

        Ok(GetBlockPermanentInfoReply {
            permanent_infos: permanent_infos_for_blocks(tables, &block_ids),
        })
    }
}

pub(super) fn permanent_infos_for_blocks(
    tables: &config::GameDB,
    block_ids: &BTreeSet<i32>,
) -> Vec<BlockPermanentInfo> {
    tables
        .room_block_color
        .iter()
        .filter(|row| block_ids.contains(&row.block_id))
        .map(|row| BlockPermanentInfo {
            block_id: Some(row.block_id),
            color: Some(row.block_color),
        })
        .collect()
}

impl RoomManager {
    pub async fn building_info(&self, db: &SqlitePool) -> Result<GetBuildingInfoReply, AppError> {
        Ok(GetBuildingInfoReply {
            building_infos: buildings::get_user_buildings(db, self.player_id)
                .await?
                .into_iter()
                .map(Into::into)
                .collect(),
        })
    }

    pub async fn use_building(
        &self,
        db: &SqlitePool,
        uid: i64,
        x: i32,
        y: i32,
        rotate: i32,
    ) -> Result<UseBuildingReply, AppError> {
        let building =
            block_packages::place_building(db, self.player_id, uid, Some((x, y, rotate)))
                .await?
                .ok_or(AppError::InvalidRequest)?;

        Ok(UseBuildingReply {
            building_info: Some(building.into()),
            delete_building_infos: Vec::new(),
            delete_road_infos: Vec::new(),
        })
    }

    pub async fn unuse_building(
        &self,
        db: &SqlitePool,
        uid: i64,
    ) -> Result<UnUseBuildingReply, AppError> {
        let building = block_packages::place_building(db, self.player_id, uid, None)
            .await?
            .ok_or(AppError::InvalidRequest)?;

        Ok(UnUseBuildingReply {
            building_infos: vec![building.into()],
            road_infos: Vec::new(),
        })
    }

    pub async fn use_block(
        &self,
        db: &SqlitePool,
        block_id: i32,
        package_id: i32,
        rotate: i32,
        x: i32,
        y: i32,
    ) -> Result<UseBlockReply, AppError> {
        block_packages::use_block(db, self.player_id, block_id, package_id, rotate, x, y).await?;
        Ok(UseBlockReply {
            block_id: Some(block_id),
            rotate: Some(rotate),
            x: Some(x),
            y: Some(y),
        })
    }

    pub async fn unuse_blocks(
        &self,
        db: &SqlitePool,
        block_ids: Vec<i32>,
    ) -> Result<UnUseBlockReply, AppError> {
        block_packages::unuse_blocks(db, self.player_id, &block_ids).await?;
        Ok(UnUseBlockReply {
            block_ids,
            building_infos: Vec::new(),
            road_infos: Vec::new(),
        })
    }

    pub async fn reset_room(&self, db: &SqlitePool) -> Result<ResetRoomReply, AppError> {
        block_packages::reset_room_edit(db, self.player_id).await?;
        Ok(ResetRoomReply {
            infos: block_packages::get_blocks(db, self.player_id)
                .await?
                .into_iter()
                .map(Into::into)
                .collect(),
            block_packages: block_packages::get_block_packages(db, self.player_id)
                .await?
                .into_iter()
                .map(Into::into)
                .collect(),
            road_infos: Vec::new(),
        })
    }

    pub async fn set_water_types(
        &self,
        db: &SqlitePool,
        changes: &[(i32, i32)],
    ) -> Result<SetWaterTypeReply, AppError> {
        let infos = block_packages::set_water_types(db, self.player_id, changes)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(SetWaterTypeReply {
            block_ids: changes.iter().map(|(block_id, _)| *block_id).collect(),
            infos,
        })
    }

    pub async fn set_block_colors(
        &self,
        db: &SqlitePool,
        changes: &[(i32, i32)],
    ) -> Result<SetBlockColorReply, AppError> {
        let infos = block_packages::set_block_colors(db, self.player_id, changes)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(SetBlockColorReply {
            block_ids: changes.iter().map(|(block_id, _)| *block_id).collect(),
            infos,
        })
    }
}
