use crate::{
    error::AppError,
    logic::turnback,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::{material_get_approach::MaterialGetApproach, red_dot_id::RedDotId},
    util::push,
};
use prost::Message;
use sonettobuf::{
    AcceptAllTurnbackBonusPointRequest, BuyDoubleBonusRequest, CmdId, GetTurnbackDailyBonusRequest,
    GetTurnbackInfoRequest, TurnbackBonusPointRequest, TurnbackFirstShowRequest,
    TurnbackOnceBonusRequest, TurnbackSignInRequest,
};
pub async fn on_get_turnback_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    GetTurnbackInfoRequest::decode(&req.data[..])?;
    let reply = turnback::turnback_info(ctx.state.db, player_id, ctx.state.tables).await?;
    ctx.send_reply(CmdId::GetTurnbackInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_turnback_first_show(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = TurnbackFirstShowRequest::decode(&req.data[..])?;
    let reply =
        turnback::turnback_first_show(ctx.state.db, player_id, msg.id.unwrap_or_default()).await?;
    ctx.send_reply(CmdId::TurnbackFirstShowCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_turnback_once_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = TurnbackOnceBonusRequest::decode(&req.data[..])?;
    let claim =
        turnback::turnback_once_bonus(ctx.state.db, player_id, msg.id.unwrap_or_default()).await?;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;
    push::clear_red_dots(ctx, [RedDotId::TurnbackOnceBonus.id()]).await?;
    ctx.send_reply(CmdId::TurnbackOnceBonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_turnback_sign_in(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = TurnbackSignInRequest::decode(&req.data[..])?;
    let claim = turnback::turnback_sign_in(
        ctx.state.db,
        player_id,
        msg.id.unwrap_or_default(),
        msg.day.unwrap_or_default(),
    )
    .await?;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;
    push::clear_red_dots(ctx, [RedDotId::TurnbackSignIn.id()]).await?;
    ctx.send_reply(CmdId::TurnbackSignInCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_turnback_daily_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetTurnbackDailyBonusRequest::decode(&req.data[..])?;
    let claim =
        turnback::turnback_daily_bonus(ctx.state.db, player_id, msg.id.unwrap_or_default()).await?;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;
    push::clear_red_dots(ctx, [RedDotId::TurnbackDailyBonus.id()]).await?;
    ctx.send_reply(CmdId::GetTurnbackDailyBonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_turnback_bonus_point(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = TurnbackBonusPointRequest::decode(&req.data[..])?;
    let claim = turnback::turnback_bonus_point(
        ctx.state.db,
        player_id,
        msg.id.unwrap_or_default(),
        msg.bonus_point_id.unwrap_or_default(),
        ctx.state.tables,
    )
    .await?;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;
    ctx.send_reply(CmdId::TurnbackBonusPointCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_accept_all_turnback_bonus_point(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = AcceptAllTurnbackBonusPointRequest::decode(&req.data[..])?;
    let claim = turnback::accept_all_turnback_bonus_point(
        ctx.state.db,
        player_id,
        msg.id.unwrap_or_default(),
        ctx.state.tables,
    )
    .await?;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;
    push::clear_red_dots(
        ctx,
        [
            RedDotId::TurnbackLegacyTask.id(),
            RedDotId::TurnbackBattlePassBonus.id(),
            RedDotId::TurnbackBattlePassTask.id(),
        ],
    )
    .await?;
    ctx.send_reply(
        CmdId::AcceptAllTurnbackBonusPointCmd,
        claim.reply,
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_buy_double_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = BuyDoubleBonusRequest::decode(&req.data[..])?;
    let claim =
        turnback::buy_double_bonus(ctx.state.db, player_id, msg.id.unwrap_or_default()).await?;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;
    ctx.send_reply(CmdId::BuyDoubleBonusCmd, claim.reply, 0, req.up_tag)
        .await
}
