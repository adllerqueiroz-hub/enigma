use super::*;

const COMMON_GROUP_COUNT: i32 = 4;
const HERO_SLOTS: i32 = 4;
const DEFAULT_CLOTH_ID: i32 = 0;
const HERO_TOUCH_COUNT_CONFIG_ID: i32 = 32;

pub async fn load_hero_touch_count(tx: &mut Transaction<'_, Sqlite>, uid: i64) -> sqlx::Result<()> {
    let game_data = configs::get();
    // Initialize touch count
    sqlx::query("INSERT INTO hero_touch_count (user_id, touch_count_left) VALUES (?, ?)")
        .bind(uid)
        .bind(
            game_data
                .r#const
                .get(HERO_TOUCH_COUNT_CONFIG_ID)
                .and_then(|row| row.value.parse::<i32>().ok())
                .unwrap_or_default(),
        )
        .execute(&mut **tx)
        .await?;

    Ok(())
}

pub async fn load_starter_hero_groups(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
) -> sqlx::Result<()> {
    let now = common::time::ServerTime::now_ms();
    for group_id in 1..=COMMON_GROUP_COUNT {
        let group = sqlx::query(
            "INSERT INTO hero_groups_common
                (user_id, group_id, name, cloth_id, assist_boss_id, created_at, updated_at)
             VALUES (?, ?, '', ?, 0, ?, ?)",
        )
        .bind(user_id)
        .bind(group_id)
        .bind(DEFAULT_CLOTH_ID)
        .bind(now)
        .bind(now)
        .execute(&mut **tx)
        .await?
        .last_insert_rowid();

        for position in 0..HERO_SLOTS {
            sqlx::query(
                "INSERT INTO hero_group_members (hero_group_id, hero_uid, position)
                 VALUES (?, 0, ?)",
            )
            .bind(group)
            .bind(position)
            .execute(&mut **tx)
            .await?;

            sqlx::query(
                "INSERT INTO hero_group_equips (hero_group_id, index_slot, equip_uid)
                 VALUES (?, ?, 0)",
            )
            .bind(group)
            .bind(position)
            .execute(&mut **tx)
            .await?;
        }
    }

    sqlx::query(
        "INSERT INTO hero_group_types
            (user_id, type_id, current_select, group_id, created_at, updated_at)
         VALUES (?, 1, 1, NULL, ?, ?)",
    )
    .bind(user_id)
    .bind(now)
    .bind(now)
    .execute(&mut **tx)
    .await?;

    tracing::info!("Created default hero groups for user {user_id}");
    Ok(())
}
