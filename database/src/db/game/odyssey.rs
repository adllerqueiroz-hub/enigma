use crate::models::game::odyssey::{
    OdysseyElementState, OdysseyItemState, OdysseyMapState, OdysseyState, OdysseyTalentState,
};
use anyhow::Result;
use sqlx::SqlitePool;

pub struct OdysseyInfoState {
    pub state: OdysseyState,
    pub maps: Vec<OdysseyMapState>,
    pub elements: Vec<OdysseyElementState>,
    pub talents: Vec<OdysseyTalentState>,
    pub items: Vec<OdysseyItemState>,
}

pub async fn sync_info(
    pool: &SqlitePool,
    user_id: i64,
    tables: &config::GameDB,
) -> Result<OdysseyInfoState> {
    let now = common::time::ServerTime::now_ms();

    sqlx::query(
        "INSERT INTO user_odyssey_state (user_id, curr_element_id, updated_at)
         VALUES (?, ?, ?)
         ON CONFLICT DO NOTHING",
    )
    .bind(user_id)
    .bind(first_unlocked_element_id(tables))
    .bind(now)
    .execute(pool)
    .await?;

    for map in tables
        .odyssey_map
        .iter()
        .filter(|map| map.unlock_condition.is_empty())
    {
        sqlx::query(
            "INSERT INTO user_odyssey_maps (user_id, map_id, updated_at)
             VALUES (?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(map.id)
        .bind(now)
        .execute(pool)
        .await?;
    }

    for element in tables
        .odyssey_element
        .iter()
        .filter(|element| element.unlock_condition.is_empty())
    {
        sqlx::query(
            "INSERT INTO user_odyssey_elements (user_id, element_id, updated_at)
             VALUES (?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(element.id)
        .bind(now)
        .execute(pool)
        .await?;
    }

    for talent in tables
        .odyssey_talent
        .iter()
        .filter(|talent| talent.level == 1 && talent.unlock_condition.is_empty())
    {
        sqlx::query(
            "INSERT INTO user_odyssey_talents (user_id, node_id, level, updated_at)
             VALUES (?, ?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(talent.node_id)
        .bind(talent.level)
        .bind(now)
        .execute(pool)
        .await?;
    }

    get_info(pool, user_id).await
}

pub async fn get_info(pool: &SqlitePool, user_id: i64) -> Result<OdysseyInfoState> {
    let state = sqlx::query_as::<_, OdysseyState>(
        "SELECT user_id, exp, level, params, curr_element_id, talent_point,
                cassandra_tree, next_mercenary_refresh, updated_at
         FROM user_odyssey_state
         WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let maps = sqlx::query_as::<_, OdysseyMapState>(
        "SELECT user_id, map_id, explore_value, updated_at
         FROM user_odyssey_maps
         WHERE user_id = ?
         ORDER BY map_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let elements = sqlx::query_as::<_, OdysseyElementState>(
        "SELECT user_id, element_id, status, updated_at
         FROM user_odyssey_elements
         WHERE user_id = ?
         ORDER BY element_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let talents = sqlx::query_as::<_, OdysseyTalentState>(
        "SELECT user_id, node_id, level, consume, updated_at
         FROM user_odyssey_talents
         WHERE user_id = ?
         ORDER BY node_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let items = sqlx::query_as::<_, OdysseyItemState>(
        "SELECT user_id, uid, item_id, count, new_flag, updated_at
         FROM user_odyssey_items
         WHERE user_id = ?
         ORDER BY uid",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(OdysseyInfoState {
        state,
        maps,
        elements,
        talents,
        items,
    })
}

fn first_unlocked_element_id(tables: &config::GameDB) -> i32 {
    tables
        .odyssey_element
        .iter()
        .filter(|element| element.unlock_condition.is_empty())
        .map(|element| element.id)
        .min()
        .unwrap_or_default()
}
