use crate::{
    error::AppError,
    logic::sign_in,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{
    CmdId, SignInAddupRequest, SignInHistoryRequest, SignInTotalRewardAllRequest,
    SignInTotalRewardRequest,
};

pub async fn on_get_sign_in_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = sign_in::get_info(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::GetSignInInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_sign_in(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let outcome = sign_in::sign_in(ctx.state.db, player_id).await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        outcome.rewards,
        outcome.material_changes,
        Some(MaterialGetApproach::SignIn),
    )
    .await?;
    ctx.send_reply(CmdId::SignInCmd, outcome.reply, 0, req.up_tag)
        .await
}

pub async fn on_sign_in_history(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SignInHistoryRequest::decode(&req.data[..])?;
    let month = msg.month.ok_or(AppError::InvalidRequest)?;
    let reply = sign_in::history(ctx.state.db, player_id, month).await?;

    ctx.send_reply(CmdId::SignInHistoryCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_sign_in_addup(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SignInAddupRequest::decode(&req.data[..])?;
    let id = msg.id.ok_or(AppError::InvalidRequest)?;
    let outcome = sign_in::addup(ctx.state.db, ctx.state.tables, player_id, id).await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        outcome.rewards,
        outcome.material_changes,
        Some(MaterialGetApproach::SignIn),
    )
    .await?;

    ctx.send_reply(CmdId::SignInAddupCmd, outcome.reply, 0, req.up_tag)
        .await
}

pub async fn on_sign_in_total_reward(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SignInTotalRewardRequest::decode(&req.data[..])?;
    let id = msg.id.ok_or(AppError::InvalidRequest)?;
    let outcome = sign_in::total_reward(ctx.state.db, ctx.state.tables, player_id, id).await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        outcome.rewards,
        outcome.material_changes,
        Some(MaterialGetApproach::LifeCircleSign),
    )
    .await?;

    ctx.send_reply(CmdId::SignInTotalRewardCmd, outcome.reply, 0, req.up_tag)
        .await
}

pub async fn on_sign_in_total_reward_all(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let _ = SignInTotalRewardAllRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let outcome = sign_in::total_reward_all(ctx.state.db, ctx.state.tables, player_id).await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        outcome.rewards,
        outcome.material_changes,
        Some(MaterialGetApproach::LifeCircleSign),
    )
    .await?;

    ctx.send_reply(CmdId::SignInTotalRewardAllCmd, outcome.reply, 0, req.up_tag)
        .await
}
