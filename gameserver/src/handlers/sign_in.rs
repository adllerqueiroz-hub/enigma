use crate::{
    error::AppError,
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
    let reply = ctx.player()?.sign_in.get_info(ctx.state.db).await?;

    ctx.send_reply(CmdId::GetSignInInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_sign_in(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let outcome = ctx.player()?.sign_in.sign_in(ctx.state.db).await?;

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
    let msg = SignInHistoryRequest::decode(&req.data[..])?;
    let month = msg.month.ok_or(AppError::InvalidRequest)?;
    let reply = ctx.player()?.sign_in.history(ctx.state.db, month).await?;

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
    let outcome = ctx
        .player()?
        .sign_in
        .addup(ctx.state.db, ctx.state.tables, id)
        .await?;

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
    let outcome = ctx
        .player()?
        .sign_in
        .total_reward(ctx.state.db, ctx.state.tables, id)
        .await?;

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
    let outcome = ctx
        .player()?
        .sign_in
        .total_reward_all(ctx.state.db, ctx.state.tables)
        .await?;

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
