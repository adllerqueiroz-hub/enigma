use super::{CostUpdate, info::manufacture_building_infos};
use crate::{error::AppError, logic::reward};
use common::types::manufacture_slot_state::ManufactureSlotState;
use database::{
    db::game::{buildings, manufacture},
    models::game::manufacture::ManufactureSlot,
};
use sonettobuf::{BuyManufactureBuildingInfoReply, ManuBuildingUpgradeReply};
use sqlx::SqlitePool;

pub async fn buy_manufacture_building(
    db: &SqlitePool,
    player_id: i64,
    building_id: i32,
    tables: &config::GameDB,
) -> Result<CostUpdate<BuyManufactureBuildingInfoReply>, AppError> {
    let config = tables
        .manufacture_building
        .get(building_id)
        .ok_or(AppError::InvalidRequest)?;
    let mut tx = db.begin().await?;
    let consumed = if config.place_no_cost == 0 {
        reward::consume(&mut tx, player_id, &reward::parse(&config.place_cost)).await?
    } else {
        reward::ConsumedRewards::default()
    };
    let building =
        buildings::create_building_in_transaction(&mut tx, player_id, building_id).await?;
    ensure_manufacture_slots(
        &mut tx,
        player_id,
        building.uid,
        config.upgrade_group_id,
        1,
        tables,
        &[],
    )
    .await?;
    tx.commit().await?;

    Ok(CostUpdate {
        reply: BuyManufactureBuildingInfoReply {
            building_id: Some(building.define_id),
            building_uid: Some(building.uid),
        },
        item_ids: consumed.item_ids,
        currency_ids: consumed.currency_ids,
        material_changes: consumed.material_changes,
    })
}

pub async fn manu_building_upgrade(
    db: &SqlitePool,
    player_id: i64,
    building_uid: i64,
    tables: &config::GameDB,
) -> Result<CostUpdate<ManuBuildingUpgradeReply>, AppError> {
    let building = buildings::get_user_buildings(db, player_id)
        .await?
        .into_iter()
        .find(|building| building.uid == building_uid)
        .ok_or(AppError::InvalidRequest)?;
    let config = tables
        .manufacture_building
        .get(building.define_id)
        .ok_or(AppError::InvalidRequest)?;
    let next_level = building.level + 1;
    let level = tables
        .manufacture_building_level
        .by_group(config.upgrade_group_id)
        .find(|level| level.id == next_level)
        .ok_or(AppError::InvalidRequest)?;
    let trade_level = manufacture::get_trade_level(db, player_id, tables).await?;
    if level.need_trade_level > trade_level {
        return Err(AppError::InvalidRequest);
    }

    let existing = manufacture::get_slots_by_building(db, player_id, building_uid).await?;
    let mut tx = db.begin().await?;
    let consumed = reward::consume(&mut tx, player_id, &reward::parse(&level.cost)).await?;
    if !buildings::upgrade_building_in_transaction(&mut tx, player_id, building_uid, building.level)
        .await?
    {
        return Err(AppError::InvalidRequest);
    }
    ensure_manufacture_slots(
        &mut tx,
        player_id,
        building_uid,
        config.upgrade_group_id,
        next_level,
        tables,
        &existing,
    )
    .await?;
    tx.commit().await?;

    Ok(CostUpdate {
        reply: ManuBuildingUpgradeReply {
            manu_building_info: manufacture_building_infos(
                db,
                player_id,
                tables,
                Some(building_uid),
            )
            .await?
            .into_iter()
            .next(),
        },
        item_ids: consumed.item_ids,
        currency_ids: consumed.currency_ids,
        material_changes: consumed.material_changes,
    })
}

async fn ensure_manufacture_slots(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    player_id: i64,
    building_uid: i64,
    upgrade_group_id: i32,
    level_id: i32,
    tables: &config::GameDB,
    existing: &[ManufactureSlot],
) -> Result<(), AppError> {
    let Some(level) = tables
        .manufacture_building_level
        .by_group(upgrade_group_id)
        .find(|level| level.id == level_id)
    else {
        return Ok(());
    };
    for slot_id in 0..level.slot_count {
        if existing.iter().any(|slot| slot.slot_id == slot_id) {
            continue;
        }
        manufacture::save_slot(
            tx,
            &ManufactureSlot {
                user_id: player_id,
                building_uid,
                slot_id,
                priority: slot_id,
                production_id: 0,
                slot_status: ManufactureSlotState::None.id(),
                inventory_count: 0,
                begin_time: 0,
                next_finish_time: 0,
                pause_time: 0,
            },
        )
        .await?;
    }
    Ok(())
}
