use anyhow::{Context, Result};
use common::time::ServerTime;
use rand::{Rng, SeedableRng, prelude::IndexedRandom, rngs::StdRng};
use sonettobuf::{GetOrderInfoReply, ProductionData, PurchaseOrderInfo, WholesaleOrderInfo};
use sqlx::{Sqlite, SqlitePool, Transaction};
use std::collections::BTreeMap;

pub async fn has_fulfillable_purchase_order(
    pool: &SqlitePool,
    user_id: i64,
    tables: &config::GameDB,
) -> Result<bool> {
    let goods = sqlx::query_as::<_, (i32, i32, i32)>(
        "SELECT goods.order_id, goods.production_id, goods.quantity
         FROM user_room_purchase_order_goods goods
         JOIN user_room_purchase_orders orders
           ON orders.user_id = goods.user_id AND orders.order_id = goods.order_id
         WHERE goods.user_id = ? AND orders.is_locked = 0
         ORDER BY goods.order_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    let inventory = super::items::get_all_items(pool, user_id)
        .await?
        .into_iter()
        .map(|item| (item.item_id, item.quantity))
        .collect::<BTreeMap<_, _>>();

    Ok(goods
        .chunk_by(|left, right| left.0 == right.0)
        .any(|order| {
            !order.is_empty()
                && order.iter().all(|(_, production_id, quantity)| {
                    tables
                        .manufacture_item
                        .get(*production_id)
                        .is_some_and(|production| {
                            inventory
                                .get(&i64::from(production.item_id))
                                .copied()
                                .unwrap_or_default()
                                >= *quantity
                        })
                })
        }))
}

pub async fn get_order_info(
    pool: &SqlitePool,
    user_id: i64,
    tables: &config::GameDB,
) -> Result<GetOrderInfoReply> {
    ensure_orders(pool, user_id, tables).await?;

    let state: (i32, i32, i32) = sqlx::query_as(
        "SELECT purchase_order_finish_count, remain_refresh_count, weekly_wholesale_revenue
         FROM user_room_order_state WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;
    let order_rows = sqlx::query_as::<_, (i32, i32, i32, bool, bool, i32, i32, bool)>(
        "SELECT order_id, last_refresh_time, buyer_id, is_advanced, is_traced,
                refresh_type, quality, is_locked
         FROM user_room_purchase_orders WHERE user_id = ? ORDER BY order_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    let goods_rows = sqlx::query_as::<_, (i32, i32, i32)>(
        "SELECT order_id, production_id, quantity
         FROM user_room_purchase_order_goods
         WHERE user_id = ? ORDER BY order_id, production_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    let mut goods = BTreeMap::<i32, Vec<ProductionData>>::new();
    for (order_id, production_id, quantity) in goods_rows {
        goods.entry(order_id).or_default().push(ProductionData {
            production_id: Some(production_id as u32),
            quantity: Some(quantity),
        });
    }
    let purchase_order_infos = order_rows
        .into_iter()
        .map(
            |(
                order_id,
                last_refresh_time,
                buyer_id,
                is_advanced,
                is_traced,
                refresh_type,
                quality,
                is_locked,
            )| PurchaseOrderInfo {
                order_id: Some(order_id),
                last_refresh_time: Some(last_refresh_time),
                buyer_id: Some(buyer_id),
                goods_info: goods.remove(&order_id).unwrap_or_default(),
                is_advanced: Some(is_advanced),
                is_traced: Some(is_traced),
                refresh_type: Some(refresh_type),
                quality: Some(quality),
                is_locked: Some(is_locked),
            },
        )
        .collect();
    let wholesale_order_infos = sqlx::query_as::<_, (i32, i32, i32)>(
        "SELECT order_id, good_id, today_sold_count
         FROM user_room_wholesale_orders WHERE user_id = ? ORDER BY order_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(order_id, good_id, today_sold_count)| WholesaleOrderInfo {
        order_id: Some(order_id),
        good_id: Some(good_id),
        today_sold_count: Some(today_sold_count),
    })
    .collect();

    Ok(GetOrderInfoReply {
        purchase_order_finish_count: Some(state.0),
        purchase_order_infos,
        wholesale_order_infos,
        remain_refresh_count: Some(state.1),
        weekly_wholesale_revenue: Some(state.2),
    })
}

pub async fn seed_orders(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    trade_level: i32,
    tables: &config::GameDB,
) -> Result<()> {
    if sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM user_room_order_state WHERE user_id = ?)",
    )
    .bind(user_id)
    .fetch_one(&mut **tx)
    .await?
    {
        return Ok(());
    }

    let refresh = tables
        .room_order_refresh
        .iter()
        .find(|row| row.level == trade_level)
        .or_else(|| tables.room_order_refresh.iter().min_by_key(|row| row.level))
        .context("room order refresh config is empty")?;
    let purchase_count = const_i32(tables, 3).unwrap_or(4).max(0);
    let remain_refresh_count = const_i32(tables, 6).unwrap_or(-1);
    sqlx::query(
        "INSERT INTO user_room_order_state
         (user_id, purchase_order_finish_count, remain_refresh_count, weekly_wholesale_revenue)
         VALUES (?, 0, ?, 0)",
    )
    .bind(user_id)
    .bind(remain_refresh_count)
    .execute(&mut **tx)
    .await?;

    let mut rng = StdRng::from_os_rng();
    let buyers = tables
        .character
        .iter()
        .filter(|character| character.is_online == "1")
        .map(|character| character.id)
        .collect::<Vec<_>>();
    let quality_pool = parse_weighted(&refresh.quality_weight);
    let last_refresh_time =
        i32::try_from(ServerTime::server_day_start_ms(ServerTime::now_ms()) / 1000)
            .unwrap_or_default();

    for order_id in 1..=purchase_count {
        let quality = weighted_pick(&quality_pool, &mut rng).context("empty order quality pool")?;
        let quality_config = tables
            .room_order_quality
            .iter()
            .find(|row| row.quality == quality)
            .context("missing room order quality")?;
        let type_counts = parse_ids(&quality_config.type_count);
        let type_count = *type_counts
            .choose(&mut rng)
            .context("empty order type count")?;
        let buyer_id = *buyers
            .choose(&mut rng)
            .context("empty room order buyer pool")?;

        sqlx::query(
            "INSERT INTO user_room_purchase_orders
             (user_id, order_id, last_refresh_time, buyer_id, is_advanced, is_traced,
              refresh_type, quality, is_locked)
             VALUES (?, ?, ?, ?, 0, 0, 1, ?, 0)",
        )
        .bind(user_id)
        .bind(order_id)
        .bind(last_refresh_time)
        .bind(buyer_id)
        .bind(quality)
        .execute(&mut **tx)
        .await?;

        let mut goods_pool = parse_weighted(&quality_config.goods_weight);
        for production_id in weighted_take(&mut goods_pool, type_count, &mut rng) {
            let item = tables
                .manufacture_item
                .get(production_id)
                .context("missing manufacture item for room order")?;
            let quantity = item.unit_count.saturating_mul(rng.random_range(1..=2));
            sqlx::query(
                "INSERT INTO user_room_purchase_order_goods
                 (user_id, order_id, production_id, quantity) VALUES (?, ?, ?, ?)",
            )
            .bind(user_id)
            .bind(order_id)
            .bind(production_id)
            .bind(quantity)
            .execute(&mut **tx)
            .await?;
        }
    }

    let mut wholesale_pool = parse_weighted(&refresh.wholesale_goods_weight);
    for (index, good_id) in weighted_take(
        &mut wholesale_pool,
        refresh.meanwhile_wholesale_num,
        &mut rng,
    )
    .into_iter()
    .enumerate()
    {
        sqlx::query(
            "INSERT INTO user_room_wholesale_orders
             (user_id, order_id, good_id, today_sold_count) VALUES (?, ?, ?, 0)",
        )
        .bind(user_id)
        .bind(index as i32 + 1)
        .bind(good_id)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

async fn ensure_orders(pool: &SqlitePool, user_id: i64, tables: &config::GameDB) -> Result<()> {
    let trade_level = super::manufacture::get_trade_level(pool, user_id, tables).await?;
    let mut tx = pool.begin().await?;
    seed_orders(&mut tx, user_id, trade_level, tables).await?;
    tx.commit().await?;
    Ok(())
}

fn const_i32(tables: &config::GameDB, id: i32) -> Option<i32> {
    tables.room_order_const.get(id)?.value.parse().ok()
}

fn parse_ids(value: &str) -> Vec<i32> {
    value
        .split('|')
        .filter_map(|part| part.parse().ok())
        .collect()
}

fn parse_weighted(value: &str) -> Vec<(i32, i32)> {
    value
        .split('|')
        .filter_map(|part| {
            let (id, weight) = part.split_once('#')?;
            Some((id.parse().ok()?, weight.parse().ok()?))
        })
        .filter(|(_, weight)| *weight > 0)
        .collect()
}

fn weighted_pick<R: Rng + ?Sized>(pool: &[(i32, i32)], rng: &mut R) -> Option<i32> {
    let total = pool.iter().map(|(_, weight)| *weight).sum::<i32>();
    let mut roll = rng.random_range(0..total);
    for (id, weight) in pool {
        if roll < *weight {
            return Some(*id);
        }
        roll -= *weight;
    }
    None
}

fn weighted_take<R: Rng + ?Sized>(pool: &mut Vec<(i32, i32)>, count: i32, rng: &mut R) -> Vec<i32> {
    let mut picked = Vec::new();
    for _ in 0..count.max(0) {
        let Some(id) = weighted_pick(pool, rng) else {
            break;
        };
        picked.push(id);
        pool.retain(|(candidate, _)| *candidate != id);
    }
    picked
}
