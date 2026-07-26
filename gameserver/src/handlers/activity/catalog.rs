use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::red_dot_id::RedDotId,
    util::push,
};
use prost::Message;
use sonettobuf::{
    ActivityNewStageReadRequest, CmdId, GetActivityInfosRequest, GetActivityInfosWithParamRequest,
    MarkUnlockNewPhotoRedDotRequest, UnlockPermanentRequest,
};

pub async fn on_get_activity_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    GetActivityInfosRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx.player_mut()?.activity.infos(db).await?;

    ctx.send_reply(CmdId::GetActivityInfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_activity_infos_with_param(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetActivityInfosWithParamRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .infos_with_param(db, &msg.activity_ids)
        .await?;

    ctx.send_reply(CmdId::GetActivityInfosWithParamCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_activity_new_stage_read(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = ActivityNewStageReadRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .mark_new_stages_read(db, msg.id)
        .await?;

    ctx.send_reply(CmdId::ActivityNewStageReadCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_unlock_permanent(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = UnlockPermanentRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .unlock_permanent(db, msg.id)
        .await?;

    ctx.send_reply(CmdId::UnlockPermanentCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mark_unlock_new_photo_red_dot(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = MarkUnlockNewPhotoRedDotRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let (_, changed_info_ids) = ctx
        .player()?
        .red_dot
        .show(db, RedDotId::ActivityJieXiKaPhoto.id(), false)
        .await?;
    push::send_red_dot_push(
        ctx,
        RedDotId::ActivityJieXiKaPhoto.id(),
        changed_info_ids.clone(),
        true,
    )
    .await?;

    ctx.send_reply(
        CmdId::MarkUnlockNewPhotoRedDotCmd,
        sonettobuf::MarkUnlockNewPhotoRedDotReply {
            activity_id: msg.activity_id,
        },
        0,
        req.up_tag,
    )
    .await?;

    Ok(())
}
