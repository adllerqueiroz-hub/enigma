use super::hero::current_room_building_degree;
use super::*;

impl RoomManager {
    pub async fn production_line_info(
        &self,
        db: &SqlitePool,
        ids: &[i32],
    ) -> Result<ProductionLineInfoReply, AppError> {
        Ok(ProductionLineInfoReply {
            production_lines: room_ob::get_production_lines(db, self.player_id, ids)
                .await?
                .into_iter()
                .map(Into::into)
                .collect(),
        })
    }

    pub async fn start_production_line(
        &self,
        db: &SqlitePool,
        tables: &config::GameDB,
        line_id: i32,
        formula_id: Option<i32>,
        count: i32,
    ) -> Result<ProductionStart, AppError> {
        let formula_id = formula_id
            .or_else(|| {
                tables
                    .production_line
                    .get(line_id)
                    .map(|line| line.init_formula)
            })
            .ok_or(AppError::InvalidRequest)?;
        let count = count.max(1);
        let formula = tables
            .formula
            .get(formula_id)
            .ok_or(AppError::InvalidRequest)?;
        let mut costs = scaled_formula_materials(&formula.cost_material, count);
        costs.extend(scaled_formula_materials(&formula.cost_score, count));
        let mut tx = db.begin().await?;
        let consumed = reward::consume(&mut tx, self.player_id, &costs).await?;
        let production_line = room_ob::start_production_line_in_transaction(
            &mut tx,
            self.player_id,
            line_id,
            formula_id,
            count,
        )
        .await?;
        tx.commit().await?;

        Ok(ProductionStart {
            reply: StartProductionLineReply {
                production_line: Some(production_line.into()),
            },
            consumed_item_ids: consumed.item_ids,
            consumed_currency_ids: consumed.currency_ids,
            material_changes: consumed.material_changes,
        })
    }

    pub async fn gain_production_line(
        &self,
        db: &SqlitePool,
        tables: &config::GameDB,
        ids: &[i32],
    ) -> Result<ProductionGain, AppError> {
        let before = room_ob::get_production_lines(db, self.player_id, ids).await?;
        if ids
            .iter()
            .any(|id| !before.iter().any(|line| line.line_id == *id))
        {
            return Err(AppError::InvalidRequest);
        }
        let mut reward_set = reward::RewardSet::default();
        for line in &before {
            if line.finish_count <= 0 {
                continue;
            }
            let Some(formula) = tables.formula.get(line.formula_id) else {
                continue;
            };
            reward_set.extend(scaled_formula_materials(
                &formula.produce,
                line.finish_count,
            ));
        }
        let degree = current_room_building_degree(db, tables, self.player_id).await?;
        let bonus = tables.building_bonus(degree).map_or(0, |row| row.bonus);
        scale_production_rewards(&mut reward_set, 1_000 + bonus);
        let material_changes = reward_set.material_changes();
        let mut tx = db.begin().await?;
        let changed =
            room_ob::gain_production_lines_in_transaction(&mut tx, self.player_id, &before)
                .await?
                .ok_or(AppError::InvalidRequest)?;
        let rewards = reward::apply_in_transaction(&mut tx, db, self.player_id, reward_set).await?;
        tx.commit().await?;
        let production_lines = changed.into_iter().map(Into::into).collect();

        Ok(ProductionGain {
            reply: GainProductionLineReply { production_lines },
            rewards,
            material_changes,
        })
    }
}

pub(super) fn scaled_formula_materials(value: &str, count: i32) -> reward::RewardSet {
    let mut parsed = reward::parse(value);
    parsed.scale(count);
    parsed
}

pub(super) fn scale_production_rewards(rewards: &mut reward::RewardSet, permille: i32) {
    for (_, count) in &mut rewards.items {
        *count = (i64::from(*count) * i64::from(permille) / 1_000) as i32;
    }
    for (_, count) in &mut rewards.currencies {
        *count = (i64::from(*count) * i64::from(permille) / 1_000) as i32;
    }
}

impl RoomManager {
    pub async fn production_line_lv_up(
        &self,
        db: &SqlitePool,
        tables: &config::GameDB,
        line_id: i32,
        new_level: i32,
    ) -> Result<RoomCostUpdate<ProductionLineLvUpReply>, AppError> {
        let line = tables
            .production_line
            .get(line_id)
            .ok_or(AppError::InvalidRequest)?;
        let Some(level) = tables
            .production_line_level
            .by_group(line.level_group)
            .find(|level| level.id == new_level)
        else {
            return Err(AppError::InvalidRequest);
        };
        let costs = scaled_formula_materials(&level.cost, 1);
        let current_level = room_ob::get_production_lines(db, self.player_id, &[line_id])
            .await?
            .into_iter()
            .find(|line| line.line_id == line_id)
            .map(|line| line.level)
            .ok_or(AppError::InvalidRequest)?;
        let mut tx = db.begin().await?;
        let consumed = reward::consume(&mut tx, self.player_id, &costs).await?;
        let production_line = room_ob::set_production_line_level_in_transaction(
            &mut tx,
            self.player_id,
            line_id,
            current_level,
            new_level,
        )
        .await?
        .ok_or(AppError::InvalidRequest)?;
        tx.commit().await?;

        Ok(RoomCostUpdate {
            reply: ProductionLineLvUpReply {
                production_line: Some(production_line.into()),
            },
            consumed_item_ids: consumed.item_ids,
            consumed_currency_ids: consumed.currency_ids,
            material_changes: consumed.material_changes,
        })
    }

    pub async fn room_level_up(
        &self,
        db: &SqlitePool,
        tables: &config::GameDB,
    ) -> Result<RoomCostUpdate<RoomLevelUpReply>, AppError> {
        let current = block_packages::get_room_state(db, self.player_id)
            .await?
            .room_level;
        let next = current + 1;
        let Some(level) = tables.room_level(next) else {
            return Err(AppError::InvalidRequest);
        };
        let costs = scaled_formula_materials(&level.cost, 1);
        let mut tx = db.begin().await?;
        let consumed = reward::consume(&mut tx, self.player_id, &costs).await?;
        let production_lines =
            block_packages::level_up_room_in_transaction(&mut tx, self.player_id, current, next)
                .await?
                .ok_or(AppError::InvalidRequest)?
                .into_iter()
                .map(Into::into)
                .collect();
        tx.commit().await?;

        Ok(RoomCostUpdate {
            reply: RoomLevelUpReply {
                room_level: Some(next),
                production_lines,
            },
            consumed_item_ids: consumed.item_ids,
            consumed_currency_ids: consumed.currency_ids,
            material_changes: consumed.material_changes,
        })
    }

    pub async fn production_line_accelerate(
        &self,
        db: &SqlitePool,
        line_id: i32,
    ) -> Result<ProductionLineAccelerateReply, AppError> {
        Ok(ProductionLineAccelerateReply {
            production_line: Some(
                room_ob::accelerate_production_line(db, self.player_id, line_id)
                    .await?
                    .into(),
            ),
        })
    }
}
