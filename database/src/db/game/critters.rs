use crate::models::game::critter::{
    CritterInfo, CritterRecord, RestInfo, SkillInfo, TagAttributeRate,
};
use anyhow::Result;
use sqlx::SqlitePool;

pub async fn get_owned_book_skins(pool: &SqlitePool, player_id: i64) -> Result<Vec<(i32, bool)>> {
    let rows: Vec<(i32, i64)> = sqlx::query_as(
        "SELECT define_id, MAX(special_skin)
         FROM critters
         WHERE player_id = ?
         GROUP BY define_id
         ORDER BY define_id",
    )
    .bind(player_id)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(define_id, special)| (define_id, special != 0))
        .collect())
}

pub async fn get_owned_book_skin(
    pool: &SqlitePool,
    player_id: i64,
    define_id: i32,
) -> Result<Option<bool>> {
    Ok(sqlx::query_scalar::<_, bool>(
        "SELECT special_skin FROM critters
         WHERE player_id = ? AND define_id = ?
         ORDER BY special_skin DESC LIMIT 1",
    )
    .bind(player_id)
    .bind(define_id)
    .fetch_optional(pool)
    .await?)
}

pub async fn get_player_critters(pool: &SqlitePool, player_id: i64) -> Result<Vec<CritterInfo>> {
    // Get all critters for player
    let critters = sqlx::query_as::<_, CritterRecord>(
        "SELECT * FROM critters WHERE player_id = ?1 ORDER BY uid",
    )
    .bind(player_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();

    for critter in critters {
        // Get skill tags
        let tags = sqlx::query_scalar::<_, String>(
            "SELECT tag FROM critter_skills WHERE critter_uid = ?1 ORDER BY sort_order",
        )
        .bind(critter.uid)
        .fetch_all(pool)
        .await?;

        // Get tag attributes
        let tag_attributes: Vec<(i32, i32)> = sqlx::query_as(
            "SELECT attribute_id, rate FROM critter_tag_attributes WHERE critter_uid = ?1",
        )
        .bind(critter.uid)
        .fetch_all(pool)
        .await?;

        let tag_attribute_rates = tag_attributes
            .into_iter()
            .map(|(attribute_id, rate)| TagAttributeRate { attribute_id, rate })
            .collect();

        // Get rest info
        let rest_info: Option<(i64, i32)> = sqlx::query_as(
            "SELECT building_uid, rest_slot_id FROM critter_rest_info WHERE critter_uid = ?1",
        )
        .bind(critter.uid)
        .fetch_optional(pool)
        .await?;

        let rest_info = rest_info.map(|(building_uid, rest_slot_id)| RestInfo {
            building_uid,
            rest_slot_id,
        });

        result.push(CritterInfo {
            tag_attribute_rates,
            uid: critter.uid,
            define_id: critter.define_id,
            create_time: critter.create_time,
            efficiency: critter.efficiency,
            patience: critter.patience,
            lucky: critter.lucky,
            efficiency_incr_rate: critter.efficiency_incr_rate,
            patience_incr_rate: critter.patience_incr_rate,
            lucky_incr_rate: critter.lucky_incr_rate,
            special_skin: critter.special_skin,
            current_mood: critter.current_mood,
            lock: critter.is_locked,
            finish_train: critter.finish_train,
            train_info: None,
            skill_info: SkillInfo { tags },
            work_info: None,
            is_high_quality: critter.is_high_quality,
            rest_info,
            train_hero_id: critter.train_hero_id,
            total_finish_count: critter.total_finish_count,
            name: critter.name,
        });
    }

    Ok(result)
}

pub async fn save_critter(pool: &SqlitePool, player_id: i64, critter: &CritterInfo) -> Result<()> {
    let now = common::time::ServerTime::now_ms();

    // Insert/update main critter
    sqlx::query(
        "INSERT INTO critters (
            uid, player_id, define_id, create_time,
            efficiency, patience, lucky,
            efficiency_incr_rate, patience_incr_rate, lucky_incr_rate,
            special_skin, current_mood, is_locked, finish_train, is_high_quality,
            train_hero_id, total_finish_count, name,
            created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)
        ON CONFLICT(uid) DO UPDATE SET
            efficiency = excluded.efficiency,
            patience = excluded.patience,
            lucky = excluded.lucky,
            efficiency_incr_rate = excluded.efficiency_incr_rate,
            patience_incr_rate = excluded.patience_incr_rate,
            lucky_incr_rate = excluded.lucky_incr_rate,
            special_skin = excluded.special_skin,
            current_mood = excluded.current_mood,
            is_locked = excluded.is_locked,
            finish_train = excluded.finish_train,
            is_high_quality = excluded.is_high_quality,
            train_hero_id = excluded.train_hero_id,
            total_finish_count = excluded.total_finish_count,
            name = excluded.name,
            updated_at = excluded.updated_at"
    )
    .bind(critter.uid)
    .bind(player_id)
    .bind(critter.define_id)
    .bind(critter.create_time)
    .bind(critter.efficiency)
    .bind(critter.patience)
    .bind(critter.lucky)
    .bind(critter.efficiency_incr_rate)
    .bind(critter.patience_incr_rate)
    .bind(critter.lucky_incr_rate)
    .bind(critter.special_skin)
    .bind(critter.current_mood)
    .bind(critter.lock)
    .bind(critter.finish_train)
    .bind(critter.is_high_quality)
    .bind(critter.train_hero_id)
    .bind(critter.total_finish_count)
    .bind(&critter.name)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    // Delete old skills and insert new ones
    sqlx::query("DELETE FROM critter_skills WHERE critter_uid = ?1")
        .bind(critter.uid)
        .execute(pool)
        .await?;

    for (i, tag) in critter.skill_info.tags.iter().enumerate() {
        sqlx::query(
            "INSERT INTO critter_skills (critter_uid, tag, sort_order) VALUES (?1, ?2, ?3)",
        )
        .bind(critter.uid)
        .bind(tag)
        .bind(i as i32)
        .execute(pool)
        .await?;
    }

    // Delete old tag attributes and insert new ones
    sqlx::query("DELETE FROM critter_tag_attributes WHERE critter_uid = ?1")
        .bind(critter.uid)
        .execute(pool)
        .await?;

    for attr in &critter.tag_attribute_rates {
        sqlx::query(
            "INSERT INTO critter_tag_attributes (critter_uid, attribute_id, rate) VALUES (?1, ?2, ?3)"
        )
        .bind(critter.uid)
        .bind(attr.attribute_id)
        .bind(attr.rate)
        .execute(pool)
        .await?;
    }

    // Save rest info
    sqlx::query("DELETE FROM critter_rest_info WHERE critter_uid = ?1")
        .bind(critter.uid)
        .execute(pool)
        .await?;

    if let Some(rest) = &critter.rest_info {
        sqlx::query(
            "INSERT INTO critter_rest_info (critter_uid, building_uid, rest_slot_id) VALUES (?1, ?2, ?3)"
        )
        .bind(critter.uid)
        .bind(rest.building_uid)
        .bind(rest.rest_slot_id)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn get_player_critter(
    pool: &SqlitePool,
    player_id: i64,
    critter_uid: i64,
) -> Result<Option<CritterRecord>> {
    Ok(
        sqlx::query_as("SELECT * FROM critters WHERE player_id = ? AND uid = ?")
            .bind(player_id)
            .bind(critter_uid)
            .fetch_optional(pool)
            .await?,
    )
}

pub async fn rename_critter(
    pool: &SqlitePool,
    player_id: i64,
    critter_uid: i64,
    name: &str,
) -> Result<()> {
    sqlx::query(
        "UPDATE critters
         SET name = ?, updated_at = ?
         WHERE player_id = ? AND uid = ?",
    )
    .bind(name)
    .bind(common::time::ServerTime::now_ms())
    .bind(player_id)
    .bind(critter_uid)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn lock_critter(
    pool: &SqlitePool,
    player_id: i64,
    critter_uid: i64,
    lock: bool,
) -> Result<()> {
    sqlx::query(
        "UPDATE critters
         SET is_locked = ?, updated_at = ?
         WHERE player_id = ? AND uid = ?",
    )
    .bind(lock)
    .bind(common::time::ServerTime::now_ms())
    .bind(player_id)
    .bind(critter_uid)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_rest_slot(
    pool: &SqlitePool,
    player_id: i64,
    building_uid: i64,
    rest_slot_id: i32,
    critter_uid: i64,
) -> Result<()> {
    sqlx::query(
        "DELETE FROM critter_rest_info
         WHERE critter_uid IN (SELECT uid FROM critters WHERE player_id = ?)
           AND (critter_uid = ? OR (building_uid = ? AND rest_slot_id = ?))",
    )
    .bind(player_id)
    .bind(critter_uid)
    .bind(building_uid)
    .bind(rest_slot_id)
    .execute(pool)
    .await?;

    sqlx::query(
        "INSERT INTO critter_rest_info (critter_uid, building_uid, rest_slot_id)
         SELECT uid, ?, ? FROM critters WHERE player_id = ? AND uid = ?",
    )
    .bind(building_uid)
    .bind(rest_slot_id)
    .bind(player_id)
    .bind(critter_uid)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn exchange_rest_slots(
    pool: &SqlitePool,
    player_id: i64,
    building_uid: i64,
    slot_a: i32,
    slot_b: i32,
) -> Result<()> {
    let critter_a = rest_slot_critter(pool, player_id, building_uid, slot_a).await?;
    let critter_b = rest_slot_critter(pool, player_id, building_uid, slot_b).await?;

    if let Some(uid) = critter_a {
        sqlx::query(
            "UPDATE critter_rest_info
             SET rest_slot_id = ?
             WHERE critter_uid = ? AND building_uid = ?",
        )
        .bind(slot_b)
        .bind(uid)
        .bind(building_uid)
        .execute(pool)
        .await?;
    }
    if let Some(uid) = critter_b {
        sqlx::query(
            "UPDATE critter_rest_info
             SET rest_slot_id = ?
             WHERE critter_uid = ? AND building_uid = ?",
        )
        .bind(slot_a)
        .bind(uid)
        .bind(building_uid)
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn rest_slot_critter(
    pool: &SqlitePool,
    player_id: i64,
    building_uid: i64,
    rest_slot_id: i32,
) -> Result<Option<i64>> {
    Ok(sqlx::query_scalar(
        "SELECT rest.critter_uid
         FROM critter_rest_info rest
         JOIN critters critter ON critter.uid = rest.critter_uid
         WHERE critter.player_id = ? AND rest.building_uid = ? AND rest.rest_slot_id = ?",
    )
    .bind(player_id)
    .bind(building_uid)
    .bind(rest_slot_id)
    .fetch_optional(pool)
    .await?)
}
