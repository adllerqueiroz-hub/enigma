use super::*;
use std::collections::BTreeMap;

pub async fn load_starter_room(tx: &mut Transaction<'_, Sqlite>, user_id: i64) -> sqlx::Result<()> {
    let tables = configs::get();
    let room_level = tables.initial_room_level();

    sqlx::query(
        "INSERT INTO user_room_state (user_id, room_level, last_reset_time)
         VALUES (?, ?, ?)",
    )
    .bind(user_id)
    .bind(room_level)
    .bind(common::time::ServerTime::now_ms())
    .execute(&mut **tx)
    .await?;

    crate::db::game::block_packages::seed_defaults(tx, user_id).await?;

    for line in tables.production_line.iter() {
        let level = i32::from(room_level >= line.need_room_level);
        sqlx::query(
            "INSERT INTO user_room_production_lines
                (user_id, line_id, formula_id, level)
             VALUES (?, ?, ?, ?)",
        )
        .bind(user_id)
        .bind(line.id)
        .bind(line.init_formula)
        .bind(level)
        .execute(&mut **tx)
        .await?;

        if level > 0 && line.init_formula > 0 {
            sqlx::query(
                "INSERT OR IGNORE INTO user_room_formulas (user_id, formula_id, count)
                 VALUES (?, ?, 1)",
            )
            .bind(user_id)
            .bind(line.init_formula)
            .execute(&mut **tx)
            .await?;
        }
    }

    let mut skins = BTreeMap::new();
    for skin in tables
        .room_skin
        .iter()
        .filter(|skin| skin.item_id == 0 && skin.priority == "1")
    {
        skins.entry(skin.r#type).or_insert(skin.id);
    }
    for (part_id, skin_id) in skins {
        sqlx::query(
            "INSERT INTO user_room_skins (user_id, part_id, skin_id)
             VALUES (?, ?, ?)",
        )
        .bind(user_id)
        .bind(part_id)
        .bind(skin_id)
        .execute(&mut **tx)
        .await?;
    }

    let trade_level = tables
        .trade_level
        .iter()
        .map(|level| level.level)
        .min()
        .unwrap_or_default();
    crate::db::game::room_orders::seed_orders(tx, user_id, trade_level, tables)
        .await
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;

    Ok(())
}
