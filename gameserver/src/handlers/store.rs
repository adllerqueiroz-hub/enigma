use crate::util::push;
use crate::{
    error::AppError,
    logic::store,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::{material_get_approach::MaterialGetApproach, red_dot_id::RedDotId},
};
use database::db::game::tasks::TaskEvent;
use prost::Message;
use sonettobuf::{
    BuyGoodsRequest, CmdId, GetStoreInfosRequest, NewOrderRequest, ReadStoreNewReply,
    ReadStoreNewRequest,
};

pub async fn on_get_store_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetStoreInfosRequest::decode(&req.data[..])?;
    let reply = store::store_infos(ctx.state.db, player_id, &msg.store_ids).await?;

    ctx.send_reply(CmdId::GetStoreInfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_buy_goods(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = BuyGoodsRequest::decode(&req.data[..])?;
    let result = store::buy_goods(
        ctx.state.db,
        player_id,
        msg.store_id,
        msg.goods_id,
        msg.num,
        msg.select_cost,
    )
    .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        result.rewards,
        result.material_changes,
        Some(MaterialGetApproach::StorePurchase),
    )
    .await?;
    crate::util::task_events::notify(
        ctx,
        player_id,
        TaskEvent::StoreGoodsBought {
            store_id: result.reply.store_id,
        },
    )
    .await?;
    if let Some(dot) = database::db::game::red_dots::hide_visible_red_dot_info(
        ctx.state.db,
        player_id,
        RedDotId::StoreTab.id(),
        result.reply.goods_id,
    )
    .await?
    {
        ctx.push_red_dot_value(
            RedDotId::StoreTab.id(),
            vec![result.reply.goods_id],
            false,
            0,
            dot.time as i32,
        )
        .await?;
    }
    ctx.send_reply(CmdId::BuyGoodsCmd, result.reply, 0, req.up_tag)
        .await
}

pub async fn on_new_order(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = NewOrderRequest::decode(&req.data[..])?;
    let result = store::new_order(
        ctx.state.db,
        player_id,
        msg.id.ok_or(AppError::InvalidRequest)?,
        msg.origin_currency,
        &msg.selection_infos,
    )
    .await?;

    ctx.send_reply(CmdId::NewOrderCmd, result.reply, 0, req.up_tag)
        .await?;
    push::send_room_reward_pushes(ctx, &result.rewards).await?;
    push::send_item_change_push(
        ctx,
        player_id,
        result.rewards.item_ids,
        result.rewards.power_item_ids,
        result.rewards.insight_item_ids,
    )
    .await?;
    push::send_currency_change_push(ctx, player_id, result.rewards.currency_ids).await?;
    push::send_equip_update_push(ctx, player_id, result.rewards.equip_uids).await?;
    push::send_hero_update_push(ctx, player_id, result.rewards.hero_ids).await?;
    push::send_skin_gain_pushes(
        ctx,
        &result.rewards.skin_gains,
        Some(MaterialGetApproach::Charge),
    )
    .await?;
    push::send_bp_score_update_pushes(ctx, &result.rewards.bp_scores).await?;
    ctx.notify(CmdId::OrderCompletePushCmd, result.complete)
        .await?;
    if let Some(push) = result.bp_pay {
        ctx.notify(CmdId::BpPayPushCmd, push).await?;
    }
    if let Some(push) = result.bp_score {
        ctx.notify(CmdId::BpScoreUpdatePushCmd, push).await?;
    }
    ctx.notify(CmdId::StatInfoPushCmd, result.stat).await?;
    push::send_material_change_push(ctx, result.material_changes, None).await?;
    Ok(())
}

pub async fn on_read_store_new(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ReadStoreNewRequest::decode(&req.data[..])?;
    let mut goods_ids = msg.goods_ids.to_vec();
    goods_ids.sort_unstable();
    goods_ids.dedup();

    database::db::game::red_dots::hide_red_dot_infos(
        ctx.state.db,
        player_id,
        RedDotId::StoreGoodsRead.id(),
        goods_ids.clone(),
    )
    .await?;

    ctx.push_red_dot(RedDotId::StoreGoodsRead.id(), vec![0], true)
        .await?;
    ctx.send_reply(
        CmdId::ReadStoreNewCmd,
        ReadStoreNewReply {
            goods_ids: msg.goods_ids,
        },
        0,
        req.up_tag,
    )
    .await
}
