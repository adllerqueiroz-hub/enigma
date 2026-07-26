use crate::error::AppError;
use database::db::game::{buildings, manufacture};
use sonettobuf::{
    DispatchCritterInfo, GetFrozenItemInfoReply, GetManufactureInfoReply, M2qEntry,
    ManuBuildingInfo, RestBuildingInfo, SlotInfo,
};
use sqlx::SqlitePool;
use std::collections::BTreeMap;

pub(super) async fn manufacture_info(
    db: &SqlitePool,
    player_id: i64,
    tables: &config::GameDB,
) -> Result<GetManufactureInfoReply, AppError> {
    Ok(GetManufactureInfoReply {
        trade_level: Some(manufacture::get_trade_level(db, player_id, tables).await?),
        manu_building_infos: manufacture_building_infos(db, player_id, tables, None).await?,
        rest_building_infos: rest_building_infos(db, player_id).await?,
        frozen_items2_count: frozen_item_entries(db, player_id).await?,
    })
}

pub(super) async fn frozen_item_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetFrozenItemInfoReply, AppError> {
    Ok(GetFrozenItemInfoReply {
        frozen_items2_count: frozen_item_entries(db, player_id).await?,
    })
}

pub(super) async fn manufacture_building_infos(
    db: &SqlitePool,
    player_id: i64,
    tables: &config::GameDB,
    only_uid: Option<i64>,
) -> Result<Vec<ManuBuildingInfo>, AppError> {
    let slots = manufacture::get_slots(db, player_id).await?;
    let mut slots_by_building = BTreeMap::<i64, Vec<SlotInfo>>::new();
    for slot in slots {
        slots_by_building
            .entry(slot.building_uid)
            .or_default()
            .push(SlotInfo {
                slot_id: Some(slot.slot_id),
                priority: Some(slot.priority),
                production_id: Some(slot.production_id),
                slot_status: Some(slot.slot_status),
                inventory_count: Some(slot.inventory_count),
                begin_time: Some(slot.begin_time),
                next_finish_time: Some(slot.next_finish_time),
                pause_time: Some(slot.pause_time),
            });
    }

    Ok(buildings::get_placed_buildings(db, player_id)
        .await?
        .into_iter()
        .filter(|building| only_uid.is_none_or(|uid| building.uid == uid))
        .filter(|building| {
            tables
                .manufacture_building
                .get(building.define_id)
                .is_some()
        })
        .map(|building| ManuBuildingInfo {
            building_uid: Some(building.uid),
            slot_infos: slots_by_building.remove(&building.uid).unwrap_or_default(),
            critter_infos: Vec::new(),
        })
        .collect())
}

async fn rest_building_infos(
    db: &SqlitePool,
    player_id: i64,
) -> Result<Vec<RestBuildingInfo>, AppError> {
    let mut rest_by_building = BTreeMap::<i64, Vec<DispatchCritterInfo>>::new();
    for rest in manufacture::get_rest_critters(db, player_id).await? {
        rest_by_building
            .entry(rest.building_uid)
            .or_default()
            .push(DispatchCritterInfo {
                critter_slot_id: Some(rest.rest_slot_id),
                critter_uid: Some(rest.critter_uid),
            });
    }

    Ok(rest_by_building
        .into_iter()
        .map(|(building_uid, unlock_slot_infos)| RestBuildingInfo {
            building_uid: Some(building_uid),
            unlock_slot_infos,
        })
        .collect())
}

async fn frozen_item_entries(db: &SqlitePool, player_id: i64) -> Result<Vec<M2qEntry>, AppError> {
    Ok(manufacture::get_frozen_items(db, player_id)
        .await?
        .into_iter()
        .map(|item| M2qEntry {
            material_id: Some(item.material_id as u32),
            quantity: Some(item.quantity),
            time: Some(item.time),
        })
        .collect())
}
