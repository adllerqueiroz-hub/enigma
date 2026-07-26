use super::{CostUpdate, info::manufacture_building_infos};
use crate::{error::AppError, reward};
use common::types::{
    manufacture_slot_operation::ManufactureSlotOperation,
    manufacture_slot_state::ManufactureSlotState,
};
use database::{db::game::manufacture, models::game::manufacture::ManufactureSlot};
use sonettobuf::{
    ManufactureAccelerateReply, MaterialData, OperationInfo, ReapFinishSlotReply,
    SelectSlotProductionPlanReply,
};
use sqlx::{Sqlite, SqlitePool, Transaction};

pub(super) async fn select_slot_production_plan(
    db: &SqlitePool,
    player_id: i64,
    building_uid: i64,
    operation_infos: &[OperationInfo],
    tables: &config::GameDB,
) -> Result<SelectSlotProductionPlanReply, AppError> {
    let mut tx = db.begin().await?;
    for info in operation_infos {
        let operation = ManufactureSlotOperation::from_id(info.operation.unwrap_or_default())
            .ok_or(AppError::InvalidRequest)?;
        let slot_id = info.slot_id.unwrap_or_default();
        match operation {
            ManufactureSlotOperation::Add => {
                let production_id = info.production_id.unwrap_or_default();
                let production = tables
                    .manufacture_item
                    .get(production_id)
                    .ok_or(AppError::InvalidRequest)?;
                manufacture::save_slot(
                    &mut tx,
                    &ManufactureSlot {
                        user_id: player_id,
                        building_uid,
                        slot_id,
                        priority: info.priority.unwrap_or_default(),
                        production_id,
                        slot_status: ManufactureSlotState::Wait.id(),
                        inventory_count: production.unit_count,
                        begin_time: 0,
                        next_finish_time: 0,
                        pause_time: 0,
                    },
                )
                .await?;
            }
            ManufactureSlotOperation::Cancel => {
                manufacture::delete_slot(&mut tx, player_id, building_uid, slot_id).await?;
            }
            ManufactureSlotOperation::MoveTop => {
                move_slot(&mut tx, player_id, building_uid, slot_id, 0).await?;
            }
            ManufactureSlotOperation::MoveBottom => {
                move_slot(&mut tx, player_id, building_uid, slot_id, i32::MAX).await?;
            }
        }
    }
    normalize_manufacture_slots(&mut tx, player_id, building_uid, tables).await?;
    tx.commit().await?;

    Ok(SelectSlotProductionPlanReply {
        manu_building_infos: manufacture_building_infos(db, player_id, tables, Some(building_uid))
            .await?,
    })
}

pub(super) async fn manufacture_accelerate(
    db: &SqlitePool,
    player_id: i64,
    building_uid: i64,
    slot_id: i32,
    use_item_data: Option<MaterialData>,
    tables: &config::GameDB,
) -> Result<CostUpdate<ManufactureAccelerateReply>, AppError> {
    let mut tx = db.begin().await?;
    let consumed = consume_material_data(&mut tx, player_id, use_item_data).await?;
    if !manufacture::complete_slot(&mut tx, player_id, building_uid, slot_id).await? {
        return Err(AppError::InvalidRequest);
    }
    tx.commit().await?;

    Ok(CostUpdate {
        reply: ManufactureAccelerateReply {
            manu_building_infos: manufacture_building_infos(
                db,
                player_id,
                tables,
                Some(building_uid),
            )
            .await?,
        },
        item_ids: consumed.item_ids,
        currency_ids: consumed.currency_ids,
        material_changes: consumed.material_changes,
    })
}

pub(super) async fn reap_finish_slot(
    db: &SqlitePool,
    player_id: i64,
    building_uid: i64,
    tables: &config::GameDB,
) -> Result<ReapFinishSlotReply, AppError> {
    let mut tx = db.begin().await?;
    let slots =
        manufacture::get_slots_by_building_in_transaction(&mut tx, player_id, building_uid).await?;
    let mut normal_reap_item = Vec::new();
    for slot in slots
        .iter()
        .filter(|slot| slot.slot_status == ManufactureSlotState::Complete.id())
    {
        let Some(production) = tables.manufacture_item.get(slot.production_id) else {
            continue;
        };
        let quantity = slot.inventory_count.max(production.unit_count);
        manufacture::add_frozen_item(&mut tx, player_id, production.item_id, quantity).await?;
        manufacture::delete_slot(&mut tx, player_id, building_uid, slot.slot_id).await?;
        normal_reap_item.push(MaterialData {
            materil_type: Some(1),
            materil_id: Some(production.item_id as u32),
            quantity: Some(quantity),
        });
    }
    normalize_manufacture_slots(&mut tx, player_id, building_uid, tables).await?;
    tx.commit().await?;

    Ok(ReapFinishSlotReply {
        building_uid: Some(building_uid),
        manu_building_infos: manufacture_building_infos(db, player_id, tables, Some(building_uid))
            .await?,
        normal_reap_item,
        cri_reap_item: Vec::new(),
        occupied_cri_item: Vec::new(),
    })
}

async fn move_slot(
    tx: &mut Transaction<'_, Sqlite>,
    player_id: i64,
    building_uid: i64,
    slot_id: i32,
    priority: i32,
) -> Result<(), AppError> {
    let mut slots =
        manufacture::get_slots_by_building_in_transaction(tx, player_id, building_uid).await?;
    let Some(slot) = slots.iter_mut().find(|slot| slot.slot_id == slot_id) else {
        return Ok(());
    };
    slot.priority = priority;
    manufacture::save_slot(tx, slot).await?;
    Ok(())
}

async fn normalize_manufacture_slots(
    tx: &mut Transaction<'_, Sqlite>,
    player_id: i64,
    building_uid: i64,
    tables: &config::GameDB,
) -> Result<(), AppError> {
    let mut slots =
        manufacture::get_slots_by_building_in_transaction(tx, player_id, building_uid).await?;
    slots.sort_by_key(|slot| (slot.priority, slot.slot_id));
    let mut running = false;
    let mut priority = 0;
    for mut slot in slots {
        if slot.production_id == 0 {
            continue;
        }
        slot.priority = priority;
        priority += 1;
        if slot.slot_status == ManufactureSlotState::Complete.id()
            || slot.slot_status == ManufactureSlotState::Stop.id()
        {
            manufacture::save_slot(tx, &slot).await?;
            continue;
        }
        if running {
            slot.slot_status = ManufactureSlotState::Wait.id();
            slot.begin_time = 0;
            slot.next_finish_time = 0;
            slot.pause_time = 0;
        } else {
            running = true;
            let production = tables
                .manufacture_item
                .get(slot.production_id)
                .ok_or(AppError::InvalidRequest)?;
            let now = (common::time::ServerTime::now_ms() / 1000) as i32;
            slot.slot_status = ManufactureSlotState::Running.id();
            slot.begin_time = now;
            slot.next_finish_time = now + production.need_time;
            slot.pause_time = 0;
        }
        manufacture::save_slot(tx, &slot).await?;
    }
    Ok(())
}

async fn consume_material_data(
    tx: &mut Transaction<'_, Sqlite>,
    player_id: i64,
    material: Option<MaterialData>,
) -> Result<reward::ConsumedRewards, AppError> {
    let Some(material) = material else {
        return Ok(reward::ConsumedRewards::default());
    };
    let material_type = material.materil_type.unwrap_or_default();
    let material_id = material.materil_id.unwrap_or_default();
    let quantity = material.quantity.unwrap_or_default();
    if quantity <= 0 {
        return Err(AppError::InvalidRequest);
    }

    let mut rewards = reward::RewardSet::default();
    match material_type {
        1 | 11 | 14 => rewards.items.push((material_id, quantity)),
        2 | 13 => rewards.currencies.push((material_id as i32, quantity)),
        _ => return Err(AppError::InvalidRequest),
    }
    reward::consume(tx, player_id, &rewards).await
}
