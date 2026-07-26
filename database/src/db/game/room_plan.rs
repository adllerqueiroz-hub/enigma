use super::{block_packages, buildings, room_ob};
use anyhow::Result;
use sonettobuf::{GetRoomPlanInfoReply, RoomLogInfo, RoomPlanInfo};
use sqlx::{Sqlite, SqlitePool, Transaction};

pub async fn get_room_plan_info(pool: &SqlitePool, user_id: i64) -> Result<GetRoomPlanInfoReply> {
    let rows = sqlx::query_as::<
        _,
        (
            i32,
            String,
            i32,
            String,
            String,
            String,
            i32,
            i32,
            String,
            i32,
        ),
    >(
        r#"
        SELECT plan_id, name, cover_id, block_infos, building_infos, skins,
               building_degree, block_count, share_code, use_count
        FROM user_room_plans
        WHERE user_id = ?
        ORDER BY plan_id
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut plans = Vec::with_capacity(rows.len());
    let mut total_use_count = 0;

    for row in rows {
        let use_count = row.9;
        total_use_count += use_count;
        plans.push(plan_from_row(row)?);
    }

    let (can_share_count, can_use_share_count) = get_room_plan_limits(pool, user_id).await?;

    Ok(GetRoomPlanInfoReply {
        infos: plans,
        can_share_count: Some(can_share_count),
        can_use_share_count: Some(can_use_share_count),
        total_use_count: Some(total_use_count),
    })
}

pub async fn get_room_plan_limits(pool: &SqlitePool, user_id: i64) -> Result<(i32, i32)> {
    Ok(sqlx::query_as(
        "SELECT can_share_count, can_use_share_count FROM user_room_state WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?)
}

pub async fn get_room_plan(
    pool: &SqlitePool,
    user_id: i64,
    plan_id: i32,
) -> Result<Option<RoomPlanInfo>> {
    let row = sqlx::query_as::<
        _,
        (
            i32,
            String,
            i32,
            String,
            String,
            String,
            i32,
            i32,
            String,
            i32,
        ),
    >(
        r#"
        SELECT plan_id, name, cover_id, block_infos, building_infos, skins,
               building_degree, block_count, share_code, use_count
        FROM user_room_plans
        WHERE user_id = ? AND plan_id = ?
        "#,
    )
    .bind(user_id)
    .bind(plan_id)
    .fetch_optional(pool)
    .await?;

    row.map(plan_from_row).transpose()
}

pub async fn get_room_share(
    pool: &SqlitePool,
    share_code: &str,
) -> Result<Option<(i64, RoomPlanInfo)>> {
    let row = sqlx::query_as::<
        _,
        (
            i64,
            i32,
            String,
            i32,
            String,
            String,
            String,
            i32,
            i32,
            String,
            i32,
        ),
    >(
        r#"
        SELECT user_id, plan_id, name, cover_id, block_infos, building_infos, skins,
               building_degree, block_count, share_code, use_count
        FROM user_room_plans
        WHERE share_code = ?
        "#,
    )
    .bind(share_code)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Ok(None);
    };
    let user_id = row.0;
    let info = plan_from_row((
        row.1, row.2, row.3, row.4, row.5, row.6, row.7, row.8, row.9, row.10,
    ))?;
    Ok(Some((user_id, info)))
}

pub async fn save_room_plan(pool: &SqlitePool, user_id: i64, info: &RoomPlanInfo) -> Result<()> {
    let mut tx = pool.begin().await?;
    save_room_plan_in_transaction(&mut tx, user_id, info).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn save_room_plan_in_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    info: &RoomPlanInfo,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO user_room_plans (
            user_id, plan_id, name, cover_id, block_infos, building_infos, skins,
            building_degree, block_count, share_code, use_count
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(user_id, plan_id) DO UPDATE SET
            name = excluded.name,
            cover_id = excluded.cover_id,
            block_infos = excluded.block_infos,
            building_infos = excluded.building_infos,
            skins = excluded.skins,
            building_degree = excluded.building_degree,
            block_count = excluded.block_count
        "#,
    )
    .bind(user_id)
    .bind(info.id.unwrap_or_default())
    .bind(info.name.clone().unwrap_or_default())
    .bind(info.cover_id.unwrap_or_default())
    .bind(serde_json::to_string(&info.infos)?)
    .bind(serde_json::to_string(&info.building_infos)?)
    .bind(serde_json::to_string(&info.skins)?)
    .bind(info.building_degree.unwrap_or_default())
    .bind(info.block_count.unwrap_or_default())
    .bind(info.share_code.clone().unwrap_or_default())
    .bind(info.use_count.unwrap_or_default())
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn update_room_plan_name(
    pool: &SqlitePool,
    user_id: i64,
    plan_id: i32,
    name: &str,
) -> Result<()> {
    sqlx::query("UPDATE user_room_plans SET name = ? WHERE user_id = ? AND plan_id = ?")
        .bind(name)
        .bind(user_id)
        .bind(plan_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_room_plan_cover(
    pool: &SqlitePool,
    user_id: i64,
    plan_id: i32,
    cover_id: i32,
) -> Result<()> {
    sqlx::query("UPDATE user_room_plans SET cover_id = ? WHERE user_id = ? AND plan_id = ?")
        .bind(cover_id)
        .bind(user_id)
        .bind(plan_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_room_plan(pool: &SqlitePool, user_id: i64, plan_id: i32) -> Result<()> {
    sqlx::query("DELETE FROM user_room_plans WHERE user_id = ? AND plan_id = ?")
        .bind(user_id)
        .bind(plan_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn switch_room_plans(
    pool: &SqlitePool,
    user_id: i64,
    id_a: i32,
    id_b: i32,
) -> Result<()> {
    let mut tx = pool.begin().await?;
    switch_room_plans_in_transaction(&mut tx, user_id, id_a, id_b).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn switch_room_plans_in_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    id_a: i32,
    id_b: i32,
) -> Result<()> {
    let count: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM user_room_plans
         WHERE user_id = ? AND plan_id IN (?, ?)",
    )
    .bind(user_id)
    .bind(id_a)
    .bind(id_b)
    .fetch_one(&mut **tx)
    .await?;
    if id_a == id_b || count != 2 {
        anyhow::bail!("both room plans must exist and be distinct");
    }
    let temp_id = -1_000_000 - id_a;
    sqlx::query("UPDATE user_room_plans SET plan_id = ? WHERE user_id = ? AND plan_id = ?")
        .bind(temp_id)
        .bind(user_id)
        .bind(id_a)
        .execute(&mut **tx)
        .await?;
    sqlx::query("UPDATE user_room_plans SET plan_id = ? WHERE user_id = ? AND plan_id = ?")
        .bind(id_a)
        .bind(user_id)
        .bind(id_b)
        .execute(&mut **tx)
        .await?;
    sqlx::query("UPDATE user_room_plans SET plan_id = ? WHERE user_id = ? AND plan_id = ?")
        .bind(id_b)
        .bind(user_id)
        .bind(temp_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub async fn save_copied_room_plan(
    pool: &SqlitePool,
    user_id: i64,
    source_user_id: i64,
    source_plan_id: i32,
    info: &mut RoomPlanInfo,
) -> Result<Option<i32>> {
    let mut tx = pool.begin().await?;
    let consumed = sqlx::query(
        "UPDATE user_room_state
         SET can_use_share_count = can_use_share_count - 1
         WHERE user_id = ? AND can_use_share_count > 0",
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await?;
    if consumed.rows_affected() != 1 {
        return Ok(None);
    }
    let source_updated = sqlx::query(
        "UPDATE user_room_plans SET use_count = use_count + 1
         WHERE user_id = ? AND plan_id = ?",
    )
    .bind(source_user_id)
    .bind(source_plan_id)
    .execute(&mut *tx)
    .await?;
    if source_updated.rows_affected() != 1 {
        return Ok(None);
    }
    if info.id == Some(0) {
        apply_room_layout(&mut tx, user_id, info).await?;
    }
    save_room_plan_in_transaction(&mut tx, user_id, info).await?;
    let remaining =
        sqlx::query_scalar("SELECT can_use_share_count FROM user_room_state WHERE user_id = ?")
            .bind(user_id)
            .fetch_one(&mut *tx)
            .await?;
    tx.commit().await?;
    Ok(Some(remaining))
}

pub async fn save_room_plan_with_layout(
    pool: &SqlitePool,
    user_id: i64,
    info: &mut RoomPlanInfo,
) -> Result<()> {
    let mut tx = pool.begin().await?;
    if info.id == Some(0) {
        apply_room_layout(&mut tx, user_id, info).await?;
    }
    save_room_plan_in_transaction(&mut tx, user_id, info).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn switch_active_room_plan(
    pool: &SqlitePool,
    user_id: i64,
    previous: &RoomPlanInfo,
    selected: &mut RoomPlanInfo,
) -> Result<()> {
    let mut tx = pool.begin().await?;
    apply_room_layout(&mut tx, user_id, selected).await?;
    save_room_plan_in_transaction(&mut tx, user_id, previous).await?;
    save_room_plan_in_transaction(&mut tx, user_id, selected).await?;
    tx.commit().await?;
    Ok(())
}

async fn apply_room_layout(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    info: &mut RoomPlanInfo,
) -> Result<()> {
    block_packages::replace_room_blocks(tx, user_id, &info.infos).await?;
    info.building_infos =
        buildings::replace_building_placement(tx, user_id, &info.building_infos).await?;
    room_ob::replace_skins(tx, user_id, &info.skins).await
}

pub async fn share_room_plan(
    pool: &SqlitePool,
    user_id: i64,
    plan_id: i32,
    share_code: &str,
) -> Result<Option<i32>> {
    let mut tx = pool.begin().await?;
    let consumed = sqlx::query(
        "UPDATE user_room_state
         SET can_share_count = can_share_count - 1
         WHERE user_id = ? AND can_share_count > 0",
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await?
    .rows_affected();
    if consumed == 0 {
        return Ok(None);
    }

    let updated =
        sqlx::query("UPDATE user_room_plans SET share_code = ? WHERE user_id = ? AND plan_id = ?")
            .bind(share_code)
            .bind(user_id)
            .bind(plan_id)
            .execute(&mut *tx)
            .await?
            .rows_affected();
    if updated == 0 {
        return Ok(None);
    }

    let remaining =
        sqlx::query_scalar("SELECT can_share_count FROM user_room_state WHERE user_id = ?")
            .bind(user_id)
            .fetch_one(&mut *tx)
            .await?;
    tx.commit().await?;
    Ok(Some(remaining))
}

pub async fn set_share_code(
    pool: &SqlitePool,
    user_id: i64,
    plan_id: i32,
    share_code: &str,
) -> Result<()> {
    sqlx::query("UPDATE user_room_plans SET share_code = ? WHERE user_id = ? AND plan_id = ?")
        .bind(share_code)
        .bind(user_id)
        .bind(plan_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_room_logs(pool: &SqlitePool, user_id: i64) -> Result<Vec<RoomLogInfo>> {
    let rows = sqlx::query_as::<_, (i32, i32, i32, i32, bool)>(
        r#"
        SELECT id, type, time, hero_id, is_new
        FROM user_room_logs
        WHERE user_id = ?
        ORDER BY time, log_uid
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(id, r#type, time, hero_id, is_new)| RoomLogInfo {
            id: Some(id),
            r#type: Some(r#type),
            time: Some(time),
            hero_id: Some(hero_id),
            is_new: Some(is_new),
        })
        .collect())
}

pub async fn read_room_logs(pool: &SqlitePool, user_id: i64, ids: &[i32]) -> Result<()> {
    if ids.is_empty() {
        sqlx::query("UPDATE user_room_logs SET is_new = 0 WHERE user_id = ?")
            .bind(user_id)
            .execute(pool)
            .await?;
        return Ok(());
    }

    let placeholders = std::iter::repeat_n("?", ids.len())
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        "UPDATE user_room_logs SET is_new = 0 WHERE user_id = ? AND id IN ({placeholders})"
    );
    let mut query = sqlx::query(&sql).bind(user_id);
    for id in ids {
        query = query.bind(id);
    }
    query.execute(pool).await?;
    Ok(())
}

fn plan_from_row(
    row: (
        i32,
        String,
        i32,
        String,
        String,
        String,
        i32,
        i32,
        String,
        i32,
    ),
) -> Result<RoomPlanInfo> {
    Ok(RoomPlanInfo {
        id: Some(row.0),
        infos: serde_json::from_str(&row.3)?,
        building_infos: serde_json::from_str(&row.4)?,
        cover_id: Some(row.2),
        name: Some(row.1),
        building_degree: Some(row.6),
        block_count: Some(row.7),
        share_code: Some(row.8),
        use_count: Some(row.9),
        skins: serde_json::from_str(&row.5)?,
    })
}
