use crate::{
    error::AppError,
    logic::mail,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::{material_get_approach::MaterialGetApproach, red_dot_id::RedDotId},
    util::push,
};
use prost::Message;
use sonettobuf::{
    CmdId, DeleteMailBatchRequest, GetAllMailsRequest, MailLockRequest, MarkMailJumpRequest,
    ReadMailBatchRequest, ReadMailRequest,
};

pub async fn on_get_all_mails(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    GetAllMailsRequest::decode(&req.data[..])?;
    let reply = mail::get_all(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::GetAllMailsCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mail_lock(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = MailLockRequest::decode(&req.data[..])?;
    let reply = mail::set_lock(
        ctx.state.db,
        player_id,
        msg.incr_id.ok_or(AppError::InvalidRequest)? as i64,
        msg.lock.ok_or(AppError::InvalidRequest)?,
    )
    .await?;

    ctx.send_reply(CmdId::MailLockCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_delete_mail_batch(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = DeleteMailBatchRequest::decode(&req.data[..])?;
    if msg.r#type != Some(1) {
        return Err(AppError::InvalidRequest);
    }
    let reply = mail::delete_claimed_unlocked(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::DeleteMailBatchCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mark_mail_jump(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = MarkMailJumpRequest::decode(&req.data[..])?;
    let reply = mail::mark_jump(
        ctx.state.db,
        player_id,
        msg.incr_id.ok_or(AppError::InvalidRequest)? as i64,
    )
    .await?;
    ctx.send_reply(CmdId::MarkMailJumpCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_read_mail(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ReadMailRequest::decode(&req.data[..])?;
    let incr_id = msg.incr_id.ok_or(AppError::InvalidRequest)? as i64;
    let (reply, outcome) = mail::claim_one(ctx.state.db, player_id, incr_id).await?;

    send_claim_pushes(ctx, player_id, outcome).await?;
    ctx.send_reply(CmdId::ReadMailCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_read_mail_batch(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    ReadMailBatchRequest::decode(&req.data[..])?;
    let (reply, outcome) = mail::claim_batch(ctx.state.db, player_id).await?;

    send_claim_pushes(ctx, player_id, outcome).await?;
    ctx.send_reply(CmdId::ReadMailBatchCmd, reply, 0, req.up_tag)
        .await
}

async fn send_claim_pushes(
    ctx: &mut ConnectionContext,
    player_id: i64,
    outcome: mail::MailClaimOutcome,
) -> Result<(), AppError> {
    let mail_red_dot = outcome.mail_red_dot;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        outcome.rewards,
        outcome.material_changes,
        Some(MaterialGetApproach::Mail),
    )
    .await?;
    if let Some((value, time)) = mail_red_dot {
        ctx.push_red_dot_value(RedDotId::MailBtn.id(), vec![0], true, value, time)
            .await?;
    }
    Ok(())
}
