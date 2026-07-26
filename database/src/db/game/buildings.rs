use crate::models::game::buildings::Building;
use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Transaction};

pub async fn get_user_buildings(pool: &SqlitePool, user_id: i64) -> Result<Vec<Building>> {
    let buildings = sqlx::query_as::<_, Building>(
        "SELECT * FROM user_buildings WHERE user_id = ? ORDER BY uid",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(buildings)
}

pub async fn get_placed_buildings(pool: &SqlitePool, user_id: i64) -> Result<Vec<Building>> {
    Ok(sqlx::query_as::<_, Building>(
        "SELECT * FROM user_buildings WHERE user_id = ? AND in_use = 1 ORDER BY uid",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?)
}

pub async fn save_building(pool: &SqlitePool, building: &Building) -> Result<()> {
    let now = common::time::ServerTime::now_ms();

    sqlx::query(
        r#"
        INSERT INTO user_buildings (
            uid, user_id, define_id, in_use, x, y, rotate, level, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(uid) DO UPDATE SET
            define_id = excluded.define_id,
            in_use = excluded.in_use,
            x = excluded.x,
            y = excluded.y,
            rotate = excluded.rotate,
            level = excluded.level,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(building.uid)
    .bind(building.user_id)
    .bind(building.define_id)
    .bind(building.in_use)
    .bind(building.x)
    .bind(building.y)
    .bind(building.rotate)
    .bind(building.level)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_building(pool: &SqlitePool, user_id: i64, define_id: i32) -> Result<Building> {
    let now = common::time::ServerTime::now_ms();
    let level = i32::from(
        config::configs::get()
            .room_building
            .get(define_id)
            .is_some_and(|building| building.can_level_up),
    );
    let result = sqlx::query(
        "INSERT INTO user_buildings (user_id, define_id, in_use, x, y, rotate, level, created_at, updated_at)
         VALUES (?, ?, 0, 0, 0, 0, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(define_id)
    .bind(level)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(Building {
        uid: result.last_insert_rowid(),
        user_id,
        define_id,
        in_use: false,
        x: 0,
        y: 0,
        rotate: 0,
        level,
        created_at: now,
        updated_at: now,
    })
}

pub async fn create_building_in_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    define_id: i32,
) -> Result<Building> {
    let now = common::time::ServerTime::now_ms();
    let level = i32::from(
        config::configs::get()
            .room_building
            .get(define_id)
            .is_some_and(|building| building.can_level_up),
    );
    let result = sqlx::query(
        "INSERT INTO user_buildings
             (user_id, define_id, in_use, x, y, rotate, level, created_at, updated_at)
         VALUES (?, ?, 0, 0, 0, 0, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(define_id)
    .bind(level)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    Ok(Building {
        uid: result.last_insert_rowid(),
        user_id,
        define_id,
        in_use: false,
        x: 0,
        y: 0,
        rotate: 0,
        level,
        created_at: now,
        updated_at: now,
    })
}

pub async fn update_building_position(
    pool: &SqlitePool,
    uid: i64,
    x: i32,
    y: i32,
    rotate: i32,
) -> Result<()> {
    let now = common::time::ServerTime::now_ms();

    sqlx::query("UPDATE user_buildings SET x = ?, y = ?, rotate = ?, updated_at = ? WHERE uid = ?")
        .bind(x)
        .bind(y)
        .bind(rotate)
        .bind(now)
        .bind(uid)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn use_building(
    pool: &SqlitePool,
    user_id: i64,
    uid: i64,
    x: i32,
    y: i32,
    rotate: i32,
) -> Result<Option<Building>> {
    let now = common::time::ServerTime::now_ms();

    sqlx::query(
        "UPDATE user_buildings
         SET in_use = 1, x = ?, y = ?, rotate = ?, updated_at = ?
         WHERE user_id = ? AND uid = ?",
    )
    .bind(x)
    .bind(y)
    .bind(rotate)
    .bind(now)
    .bind(user_id)
    .bind(uid)
    .execute(pool)
    .await?;

    get_building(pool, user_id, uid).await
}

pub async fn set_building_placement(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    uid: i64,
    placement: Option<(i32, i32, i32)>,
) -> Result<Option<Building>> {
    let (in_use, x, y, rotate) = placement
        .map(|(x, y, rotate)| (true, x, y, rotate))
        .unwrap_or((false, 0, 0, 0));
    sqlx::query(
        "UPDATE user_buildings
         SET in_use = ?, x = ?, y = ?, rotate = ?, updated_at = ?
         WHERE user_id = ? AND uid = ?",
    )
    .bind(in_use)
    .bind(x)
    .bind(y)
    .bind(rotate)
    .bind(common::time::ServerTime::now_ms())
    .bind(user_id)
    .bind(uid)
    .execute(&mut **tx)
    .await?;

    Ok(
        sqlx::query_as("SELECT * FROM user_buildings WHERE user_id = ? AND uid = ?")
            .bind(user_id)
            .bind(uid)
            .fetch_optional(&mut **tx)
            .await?,
    )
}

pub async fn get_placed_buildings_in_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
) -> Result<Vec<Building>> {
    Ok(
        sqlx::query_as(
            "SELECT * FROM user_buildings WHERE user_id = ? AND in_use = 1 ORDER BY uid",
        )
        .bind(user_id)
        .fetch_all(&mut **tx)
        .await?,
    )
}

pub async fn replace_building_placement(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    infos: &[sonettobuf::BuildingInfo],
) -> Result<Vec<sonettobuf::BuildingInfo>> {
    let inventory: Vec<Building> =
        sqlx::query_as("SELECT * FROM user_buildings WHERE user_id = ? ORDER BY uid")
            .bind(user_id)
            .fetch_all(&mut **tx)
            .await?;
    let mut selected = Vec::with_capacity(infos.len());

    for info in infos {
        let uid = info.uid.unwrap_or_default();
        let define_id = info.define_id.unwrap_or_default();
        let building = inventory
            .iter()
            .find(|building| building.uid == uid && !selected.contains(&building.uid))
            .or_else(|| {
                inventory.iter().find(|building| {
                    building.define_id == define_id && !selected.contains(&building.uid)
                })
            })
            .ok_or_else(|| anyhow::anyhow!("room building {define_id} is not owned"))?;
        selected.push(building.uid);
    }

    let now = common::time::ServerTime::now_ms();
    sqlx::query(
        "UPDATE user_buildings
         SET in_use = 0, x = 0, y = 0, rotate = 0, updated_at = ?
         WHERE user_id = ?",
    )
    .bind(now)
    .bind(user_id)
    .execute(&mut **tx)
    .await?;

    let mut applied = Vec::with_capacity(infos.len());
    for (info, uid) in infos.iter().zip(selected) {
        let building = set_building_placement(
            tx,
            user_id,
            uid,
            Some((
                info.x.unwrap_or_default(),
                info.y.unwrap_or_default(),
                info.rotate.unwrap_or_default(),
            )),
        )
        .await?
        .ok_or_else(|| anyhow::anyhow!("room building {uid} disappeared"))?;
        applied.push(building.into());
    }

    Ok(applied)
}

pub async fn upgrade_building(pool: &SqlitePool, uid: i64) -> Result<()> {
    let now = common::time::ServerTime::now_ms();

    sqlx::query("UPDATE user_buildings SET level = level + 1, updated_at = ? WHERE uid = ?")
        .bind(now)
        .bind(uid)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn upgrade_building_in_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    uid: i64,
    expected_level: i32,
) -> Result<bool> {
    let result = sqlx::query(
        "UPDATE user_buildings SET level = level + 1, updated_at = ?
         WHERE user_id = ? AND uid = ? AND level = ?",
    )
    .bind(common::time::ServerTime::now_ms())
    .bind(user_id)
    .bind(uid)
    .bind(expected_level)
    .execute(&mut **tx)
    .await?;
    Ok(result.rows_affected() == 1)
}

async fn get_building(pool: &SqlitePool, user_id: i64, uid: i64) -> Result<Option<Building>> {
    Ok(
        sqlx::query_as::<_, Building>("SELECT * FROM user_buildings WHERE user_id = ? AND uid = ?")
            .bind(user_id)
            .bind(uid)
            .fetch_optional(pool)
            .await?,
    )
}
