use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::{material_get_approach::MaterialGetApproach, red_dot_id::RedDotId},
    util::push,
};
use prost::Message;
use sonettobuf::{
    AutoUseExpirePowerItemRequest, BuyPowerRequest, CmdId, ExchangeDiamondRequest,
    ExchangeSameCurrencyRequest, GetBuyPowerInfoRequest, GetCurrencyListRequest, ItemChangePush,
    MarkReadSubType21Request, PopExchangeSameCurrencyRequest, UseInsightItemRequest,
    UseItemRequest, UsePowerItemListRequest, UsePowerItemRequest,
};

pub async fn on_get_currency_list(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let inventory = ctx.player()?.inventory;
    let msg = GetCurrencyListRequest::decode(&req.data[..])?;
    let reply = inventory
        .currency_list(ctx.state.db, msg.currency_ids)
        .await?;

    ctx.send_reply(CmdId::GetCurrencyListCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_exchange_same_currency(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let inventory = ctx.player()?.inventory;
    ExchangeSameCurrencyRequest::decode(&req.data[..])?;
    let reply = inventory.exchange_same_currency(ctx.state.db).await?;

    ctx.send_reply(CmdId::ExchangeSameCurrencyCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_pop_exchange_same_currency(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let inventory = ctx.player()?.inventory;
    let msg = PopExchangeSameCurrencyRequest::decode(&req.data[..])?;
    let reply = inventory
        .pop_exchange_same_currency(ctx.state.db, msg.currency_ids)
        .await?;

    ctx.send_reply(CmdId::PopExchangeSameCurrencyCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_exchange_diamond(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player = ctx.player()?;
    let player_id = player.id;
    let inventory = player.inventory;
    let msg = ExchangeDiamondRequest::decode(&req.data[..])?;
    let amount = msg.exchange_diamond.ok_or(AppError::InvalidRequest)?;
    let reply = inventory
        .exchange_diamond(
            ctx.state.db,
            amount,
            msg.op_type.ok_or(AppError::InvalidRequest)?,
        )
        .await?;

    push::send_currency_change_push(ctx, player_id, vec![(1, -amount), (2, amount)]).await?;
    ctx.send_reply(CmdId::ExchangeDiamondCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_item_list(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let inventory = ctx.player()?.inventory;
    let reply = inventory.item_list(ctx.state.db).await?;

    ctx.send_reply(CmdId::GetItemListCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_buy_power_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let inventory = ctx.player()?.inventory;
    GetBuyPowerInfoRequest::decode(&req.data[..])?;
    let reply = inventory.buy_power_info(ctx.state.db).await?;

    ctx.send_reply(CmdId::GetBuyPowerInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_buy_power(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player = ctx.player()?;
    let player_id = player.id;
    let inventory = player.inventory;
    BuyPowerRequest::decode(&req.data[..])?;
    let (reply, cost) = inventory.buy_power(ctx.state.db).await?;

    push::send_currency_change_push(ctx, player_id, vec![(cost.0, -cost.1), (4, 0)]).await?;
    ctx.send_reply(CmdId::BuyPowerCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_auto_use_expire_power_item(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    ctx.player()?;
    AutoUseExpirePowerItemRequest::decode(&req.data[..])?;
    let player = ctx.player()?;
    let player_id = player.id;
    let inventory = player.inventory;
    let reply = inventory.auto_use_expired_power_items(ctx.state.db).await?;

    if reply.used.unwrap_or(false) {
        push::send_currency_change_push(ctx, player_id, vec![(4, 0)]).await?;
    }
    ctx.send_reply(CmdId::AutoUseExpirePowerItemCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_use_power_item(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player = ctx.player()?;
    let player_id = player.id;
    let inventory = player.inventory;
    let msg = UsePowerItemRequest::decode(&req.data[..])?;
    let (reply, power_items) = inventory
        .use_power_item(ctx.state.db, msg.uid.ok_or(AppError::InvalidRequest)?)
        .await?;

    send_power_item_updates(ctx, power_items).await?;
    push::send_currency_change_push(ctx, player_id, vec![(4, 0)]).await?;
    ctx.send_reply(CmdId::UsePowerItemCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_use_power_item_list(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player = ctx.player()?;
    let player_id = player.id;
    let inventory = player.inventory;
    let msg = UsePowerItemListRequest::decode(&req.data[..])?;
    let (reply, power_items) = inventory
        .use_power_item_list(ctx.state.db, msg.use_power_item_info)
        .await?;

    send_power_item_updates(ctx, power_items).await?;
    push::send_currency_change_push(ctx, player_id, vec![(4, 0)]).await?;
    ctx.send_reply(CmdId::UsePowerItemListCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_use_item(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player = ctx.player()?;
    let player_id = player.id;
    let inventory = player.inventory;
    let msg = UseItemRequest::decode(&req.data[..])?;
    let (reply, changed, consumed, material_changes) = inventory
        .use_items(ctx.state.db, msg.entry, msg.target_id)
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        changed,
        material_changes,
        Some(MaterialGetApproach::ItemUseReward),
    )
    .await?;
    push::send_item_change_push(ctx, player_id, consumed, Vec::new(), Vec::new()).await?;
    if !reply.entry.is_empty() {
        push::send_trade_order_red_dot(ctx, player_id).await?;
    }
    ctx.send_reply(CmdId::UseItemCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_use_insight_item(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player = ctx.player()?;
    let player_id = player.id;
    let inventory = player.inventory;
    let msg = UseInsightItemRequest::decode(&req.data[..])?;
    let uid = msg.uid.ok_or(AppError::InvalidRequest)?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let (reply, item_id) = inventory
        .use_insight_item(ctx.state.db, uid, hero_id)
        .await?;

    push::send_item_change_push(ctx, player_id, Vec::new(), Vec::new(), vec![item_id]).await?;
    push::send_hero_update_push(ctx, player_id, vec![hero_id]).await?;
    ctx.send_reply(CmdId::UseInsightItemCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mark_read_sub_type21(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let inventory = ctx.player()?.inventory;
    let msg = MarkReadSubType21Request::decode(&req.data[..])?;
    let item_id = msg.item_id.ok_or(AppError::InvalidRequest)?;
    let reply = inventory
        .mark_read_sub_type21(ctx.state.db, item_id)
        .await?;

    push::send_red_dot_push(
        ctx,
        RedDotId::PlayerChangeBgItemNew.id(),
        vec![item_id],
        false,
    )
    .await?;
    ctx.send_reply(CmdId::MarkReadSubType21Cmd, reply, 0, req.up_tag)
        .await
}

async fn send_power_item_updates(
    ctx: &mut ConnectionContext,
    power_items: Vec<sonettobuf::PowerItem>,
) -> Result<(), AppError> {
    ctx.notify(
        CmdId::ItemChangePushCmd,
        ItemChangePush {
            power_items,
            ..Default::default()
        },
    )
    .await
}
