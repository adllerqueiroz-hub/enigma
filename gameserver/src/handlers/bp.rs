use crate::{
    error::AppError,
    logic::bp,
    net::{context::ConnectionContext, packet::ClientPacket},
    player::red_dot,
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{
    BpBuyLevelRequset, BpMarkFirstShowRequest, CmdId, GetBpBonusRequest, GetBpInfoRequest,
    GetSelfSelectBonusRequest,
};

pub async fn on_get_bp_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetBpInfoRequest::decode(&req.data[..])?;
    let reply = bp::get_bp_info(ctx.state.db, player_id, msg.get_task.unwrap_or(false)).await?;
    ctx.send_reply(CmdId::GetBpInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_bp_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetBpBonusRequest::decode(&req.data[..])?;
    let claim = bp::get_bp_bonus(
        ctx.state.db,
        player_id,
        msg.id,
        msg.level,
        msg.pay_bonus,
        msg.is_sp,
    )
    .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::BattlePass),
    )
    .await?;
    push::send_red_dot_groups(
        ctx,
        red_dot::battle_pass_red_dot_groups(ctx.state.db, player_id).await?,
    )
    .await?;
    ctx.send_reply(CmdId::GetBpBonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_self_select_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetSelfSelectBonusRequest::decode(&req.data[..])?;
    let claim =
        bp::get_self_select_bonus(ctx.state.db, player_id, msg.id, msg.level, msg.index).await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::BattlePass),
    )
    .await?;
    push::send_red_dot_groups(
        ctx,
        red_dot::battle_pass_red_dot_groups(ctx.state.db, player_id).await?,
    )
    .await?;
    ctx.send_reply(CmdId::GetSelfSelectBonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_buy_level(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = BpBuyLevelRequset::decode(&req.data[..])?;
    let purchase = bp::buy_levels(ctx.state.db, player_id, msg.id, msg.num).await?;

    push::send_currency_change_push(ctx, player_id, vec![purchase.currency_change]).await?;
    push::send_material_change_push(
        ctx,
        vec![purchase.material_change],
        Some(MaterialGetApproach::BattlePass),
    )
    .await?;
    push::send_red_dot_groups(
        ctx,
        red_dot::battle_pass_red_dot_groups(ctx.state.db, player_id).await?,
    )
    .await?;
    ctx.send_reply(CmdId::BpBuyLevelRequsetCmd, purchase.reply, 0, req.up_tag)
        .await
}

pub async fn on_bp_mark_first_show(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = BpMarkFirstShowRequest::decode(&req.data[..])?;
    let reply = bp::mark_first_show(ctx.state.db, player_id, msg.id, msg.is_sp).await?;

    ctx.send_reply(CmdId::BpMarkFirstShowCmd, reply, 0, req.up_tag)
        .await
}
