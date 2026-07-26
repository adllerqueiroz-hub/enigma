use super::{goods_store_id, is_time_active};
use crate::{error::AppError, logic::reward};
use common::time::ServerTime;
use database::db::game::store;
use sonettobuf::BuyGoodsReply;
use sqlx::SqlitePool;

pub struct BuyGoodsResult {
    pub reply: BuyGoodsReply,
    pub rewards: reward::AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn buy_goods(
    db: &SqlitePool,
    player_id: i64,
    store_id: i32,
    goods_id: i32,
    num: i32,
    select_cost: Option<i32>,
) -> Result<BuyGoodsResult, AppError> {
    if num <= 0 {
        return Err(AppError::InvalidRequest);
    }
    let goods = config::configs::get()
        .store_goods
        .get(goods_id)
        .ok_or(AppError::InvalidRequest)?;
    let now = ServerTime::now_ms();
    if !goods.is_online
        || goods_store_id(&goods.store_id) != Some(store_id)
        || !is_time_active(&goods.online_time, &goods.offline_time, now)
    {
        return Err(AppError::InvalidRequest);
    }
    let buy_counts = store::get_buy_counts(db, player_id).await?;
    let buy_count = buy_counts.get(&goods_id).copied().unwrap_or_default();
    let next_buy_count = buy_count.checked_add(num).ok_or(AppError::InvalidRequest)?;

    if goods.max_buy_count > 0 && next_buy_count > goods.max_buy_count {
        return Err(AppError::InvalidRequest);
    }

    let cost = match select_cost.unwrap_or(1) {
        1 => &goods.cost,
        2 if !goods.cost2.is_empty() => &goods.cost2,
        _ => return Err(AppError::InvalidRequest),
    };
    let mut tx = db.begin().await?;
    let consumed =
        reward::consume(&mut tx, player_id, &purchase_cost(cost, buy_count, num)).await?;
    store::add_buy_count_in_transaction(&mut tx, player_id, goods_id, buy_count, num)
        .await?
        .ok_or(AppError::InvalidRequest)?;
    let mut product = reward::parse(&goods.product);
    product.scale(num);
    let material_changes = product.material_changes();
    let mut rewards = reward::apply_in_transaction(&mut tx, db, player_id, product).await?;
    tx.commit().await?;
    rewards.item_ids.extend(consumed.item_ids);
    rewards.currency_ids.extend(consumed.currency_ids);

    Ok(BuyGoodsResult {
        reply: BuyGoodsReply {
            store_id,
            goods_id,
            num,
            select_cost,
        },
        rewards,
        material_changes,
    })
}

pub(crate) fn purchase_cost(cost: &str, buy_count: i32, num: i32) -> reward::RewardSet {
    let tiers = cost
        .split('|')
        .filter(|tier| !tier.is_empty())
        .collect::<Vec<_>>();
    let mut total = reward::RewardSet::default();
    let Some(last) = tiers.last() else {
        return total;
    };

    for index in buy_count..buy_count + num {
        total.extend(reward::parse(
            tiers.get(index as usize).copied().unwrap_or(*last),
        ));
    }
    total
}
