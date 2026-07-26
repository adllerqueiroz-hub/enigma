use anyhow::Result;
use sonettobuf::PowerItem;
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct PowerMakerState {
    pub status: i32,
    pub next_remain_second: i32,
    pub make_count: i32,
    pub logout_second: i32,
}

#[derive(Clone, Copy)]
enum PowerId {
    Overflow = 31,
}

impl PowerId {
    fn id(self) -> i32 {
        self as i32
    }
}

pub async fn get_state(pool: &SqlitePool, user_id: i64) -> Result<PowerMakerState> {
    let state = sqlx::query_as::<_, (i32, i32, i32, i32)>(
        r#"
        SELECT status, next_remain_second, make_count, logout_second
        FROM user_power_maker_state
        WHERE user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let Some((status, next_remain_second, make_count, logout_second)) = state else {
        return Ok(PowerMakerState {
            status: 0,
            next_remain_second: 0,
            make_count: 0,
            logout_second: 0,
        });
    };

    Ok(PowerMakerState {
        status,
        next_remain_second,
        make_count,
        logout_second,
    })
}

pub async fn get_maker_items(pool: &SqlitePool, user_id: i64) -> Result<Vec<PowerItem>> {
    let items = sqlx::query_as::<_, crate::models::game::items::PowerItem>(
        r#"
        SELECT uid, user_id, item_id, quantity, expire_time, created_at
        FROM power_items
        WHERE user_id = ?
          AND item_id = ?
          AND (expire_time = 0 OR expire_time > CAST(strftime('%s','now') AS INTEGER))
        ORDER BY expire_time, uid
        "#,
    )
    .bind(user_id)
    .bind(PowerId::Overflow.id())
    .fetch_all(pool)
    .await?;

    Ok(items.into_iter().map(Into::into).collect())
}
