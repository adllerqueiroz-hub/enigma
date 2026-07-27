use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{
    CmdId, GetOrderInfoRequest, GetTradeSupportBonusRequest, GetTradeTaskExtraBonusRequest,
    GetTradeTaskInfoRequest, ReadNewTradeTaskRequest, TradeLevelUpRequest,
};

pub async fn on_get_order_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    GetOrderInfoRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .room
        .order_info(ctx.state.db, ctx.state.tables)
        .await?;
    ctx.send_reply(CmdId::GetOrderInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_trade_task_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    GetTradeTaskInfoRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .room
        .trade_task_info(ctx.state.db, ctx.state.tables)
        .await?;
    ctx.send_reply(CmdId::GetTradeTaskInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_read_new_trade_task(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = ReadNewTradeTaskRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .room
        .read_new_trade_tasks(ctx.state.db, msg.ids)
        .await?;
    ctx.send_reply(CmdId::ReadNewTradeTaskCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_trade_task_extra_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    GetTradeTaskExtraBonusRequest::decode(&req.data[..])?;
    let reply = ctx.player()?.room.trade_task_extra_bonus();
    ctx.send_reply(CmdId::GetTradeTaskExtraBonusCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_trade_support_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetTradeSupportBonusRequest::decode(&req.data[..])?;
    let claim = ctx
        .player()?
        .room
        .trade_support_bonus(ctx.state.db, msg.id.unwrap_or_default())
        .await?;
    ctx.send_reply(CmdId::GetTradeSupportBonusCmd, claim.reply, 0, req.up_tag)
        .await?;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Task),
    )
    .await
}

pub async fn on_trade_level_up(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    TradeLevelUpRequest::decode(&req.data[..])?;
    let claim = ctx
        .player()?
        .room
        .trade_level_up(ctx.state.db, ctx.state.tables)
        .await?;
    ctx.send_reply(CmdId::TradeLevelUpCmd, claim.reply, 0, req.up_tag)
        .await?;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Task),
    )
    .await
}
