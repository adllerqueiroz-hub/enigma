use crate::models::game::manufacture::{FrozenItem, ManufactureSlot, RestCritter};
use anyhow::Result;
use common::types::manufacture_slot_state::ManufactureSlotState;
use sqlx::{Sqlite, SqlitePool, Transaction};

pub async fn get_trade_level(
    pool: &SqlitePool,
    user_id: i64,
    tables: &config::GameDB,
) -> Result<i32> {
    let default_level = tables
        .manufacture_building
        .iter()
        .map(|row| row.place_trade_level)
        .min()
        .unwrap_or_default();

    Ok(
        sqlx::query_scalar("SELECT trade_level FROM user_manufacture_state WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await?
            .unwrap_or(default_level),
    )
}

pub async fn set_trade_level(pool: &SqlitePool, user_id: i64, level: i32) -> Result<()> {
    sqlx::query(
        "INSERT INTO user_manufacture_state (user_id, trade_level, updated_at)
         VALUES (?, ?, ?)
         ON CONFLICT(user_id) DO UPDATE SET
             trade_level = excluded.trade_level,
             updated_at = excluded.updated_at",
    )
    .bind(user_id)
    .bind(level)
    .bind(common::time::ServerTime::now_ms())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_trade_level_in_transaction(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    user_id: i64,
    expected_level: i32,
    level: i32,
) -> Result<bool> {
    let result = sqlx::query(
        "UPDATE user_manufacture_state
         SET trade_level = ?, updated_at = ?
         WHERE user_id = ? AND trade_level = ?",
    )
    .bind(level)
    .bind(common::time::ServerTime::now_ms())
    .bind(user_id)
    .bind(expected_level)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn get_slots(pool: &SqlitePool, user_id: i64) -> Result<Vec<ManufactureSlot>> {
    refresh_finished_slots(pool, user_id).await?;

    Ok(sqlx::query_as::<_, ManufactureSlot>(
        "SELECT user_id, building_uid, slot_id, priority, production_id, slot_status,
                inventory_count, begin_time, next_finish_time, pause_time
         FROM user_manufacture_slots
         WHERE user_id = ?
         ORDER BY building_uid, priority, slot_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?)
}

pub async fn get_slots_by_building(
    pool: &SqlitePool,
    user_id: i64,
    building_uid: i64,
) -> Result<Vec<ManufactureSlot>> {
    refresh_finished_slots(pool, user_id).await?;

    Ok(sqlx::query_as::<_, ManufactureSlot>(
        "SELECT user_id, building_uid, slot_id, priority, production_id, slot_status,
                inventory_count, begin_time, next_finish_time, pause_time
         FROM user_manufacture_slots
         WHERE user_id = ? AND building_uid = ?
         ORDER BY priority, slot_id",
    )
    .bind(user_id)
    .bind(building_uid)
    .fetch_all(pool)
    .await?)
}

pub async fn get_slots_by_building_in_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    building_uid: i64,
) -> Result<Vec<ManufactureSlot>> {
    refresh_finished_slots_in_transaction(tx, user_id).await?;
    Ok(sqlx::query_as::<_, ManufactureSlot>(
        "SELECT user_id, building_uid, slot_id, priority, production_id, slot_status,
                inventory_count, begin_time, next_finish_time, pause_time
         FROM user_manufacture_slots
         WHERE user_id = ? AND building_uid = ?
         ORDER BY priority, slot_id",
    )
    .bind(user_id)
    .bind(building_uid)
    .fetch_all(&mut **tx)
    .await?)
}

pub async fn get_frozen_items(pool: &SqlitePool, user_id: i64) -> Result<Vec<FrozenItem>> {
    Ok(sqlx::query_as::<_, FrozenItem>(
        "SELECT user_id, material_id, quantity, time
         FROM user_manufacture_frozen_items
         WHERE user_id = ?
         ORDER BY material_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?)
}

pub async fn add_frozen_item(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    material_id: i32,
    quantity: i32,
) -> Result<()> {
    if quantity <= 0 {
        return Ok(());
    }

    sqlx::query(
        "INSERT INTO user_manufacture_frozen_items (user_id, material_id, quantity, time)
         VALUES (?, ?, ?, 0)
         ON CONFLICT(user_id, material_id) DO UPDATE SET
             quantity = quantity + excluded.quantity",
    )
    .bind(user_id)
    .bind(material_id)
    .bind(quantity)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn get_rest_critters(pool: &SqlitePool, user_id: i64) -> Result<Vec<RestCritter>> {
    Ok(sqlx::query_as::<_, RestCritter>(
        "SELECT rest.building_uid, rest.rest_slot_id, rest.critter_uid
         FROM critter_rest_info rest
         JOIN critters critter ON critter.uid = rest.critter_uid
         WHERE critter.player_id = ?
         ORDER BY rest.building_uid, rest.rest_slot_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?)
}

pub async fn save_slot(tx: &mut Transaction<'_, Sqlite>, slot: &ManufactureSlot) -> Result<()> {
    sqlx::query(
        "INSERT INTO user_manufacture_slots
         (user_id, building_uid, slot_id, priority, production_id, slot_status,
          inventory_count, begin_time, next_finish_time, pause_time)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(user_id, building_uid, slot_id) DO UPDATE SET
            priority = excluded.priority,
            production_id = excluded.production_id,
            slot_status = excluded.slot_status,
            inventory_count = excluded.inventory_count,
            begin_time = excluded.begin_time,
            next_finish_time = excluded.next_finish_time,
            pause_time = excluded.pause_time",
    )
    .bind(slot.user_id)
    .bind(slot.building_uid)
    .bind(slot.slot_id)
    .bind(slot.priority)
    .bind(slot.production_id)
    .bind(slot.slot_status)
    .bind(slot.inventory_count)
    .bind(slot.begin_time)
    .bind(slot.next_finish_time)
    .bind(slot.pause_time)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

pub async fn delete_slot(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    building_uid: i64,
    slot_id: i32,
) -> Result<()> {
    sqlx::query(
        "DELETE FROM user_manufacture_slots
         WHERE user_id = ? AND building_uid = ? AND slot_id = ?",
    )
    .bind(user_id)
    .bind(building_uid)
    .bind(slot_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn complete_slot(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    building_uid: i64,
    slot_id: i32,
) -> Result<bool> {
    let result = sqlx::query(
        "UPDATE user_manufacture_slots
         SET slot_status = ?, next_finish_time = 0, pause_time = 0
         WHERE user_id = ? AND building_uid = ? AND slot_id = ?
           AND production_id != 0 AND slot_status = ?",
    )
    .bind(ManufactureSlotState::Complete.id())
    .bind(user_id)
    .bind(building_uid)
    .bind(slot_id)
    .bind(ManufactureSlotState::Running.id())
    .execute(&mut **tx)
    .await?;

    Ok(result.rows_affected() == 1)
}

async fn refresh_finished_slots(pool: &SqlitePool, user_id: i64) -> Result<()> {
    let mut tx = pool.begin().await?;
    refresh_finished_slots_in_transaction(&mut tx, user_id).await?;
    tx.commit().await?;
    Ok(())
}

async fn refresh_finished_slots_in_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
) -> Result<()> {
    let now = (common::time::ServerTime::now_ms() / 1000) as i32;
    sqlx::query(
        "UPDATE user_manufacture_slots
         SET slot_status = ?, next_finish_time = 0, pause_time = 0
         WHERE user_id = ? AND slot_status = ? AND next_finish_time > 0 AND next_finish_time <= ?",
    )
    .bind(ManufactureSlotState::Complete.id())
    .bind(user_id)
    .bind(ManufactureSlotState::Running.id())
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
