use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{
    CmdId, GetFairylandInfoRequest, RecordDialogRequest, RecordElementRequest, ResolvePuzzleRequest,
};

pub async fn on_get_info(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    GetFairylandInfoRequest::decode(&req.data[..])?;
    let reply = ctx.player()?.fairyland.get_info(ctx.state.db).await?;
    ctx.send_reply(CmdId::GetFairylandInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_record_dialog(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = RecordDialogRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .fairyland
        .record_dialog(
            ctx.state.db,
            ctx.state.tables,
            msg.dialog_id.ok_or(AppError::InvalidRequest)?,
        )
        .await?;
    ctx.send_reply(CmdId::RecordDialogCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_record_element(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = RecordElementRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .fairyland
        .record_element(
            ctx.state.db,
            ctx.state.tables,
            msg.element_id.ok_or(AppError::InvalidRequest)?,
        )
        .await?;
    ctx.send_reply(CmdId::RecordElementCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_resolve_puzzle(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = ResolvePuzzleRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .fairyland
        .resolve_puzzle(
            ctx.state.db,
            ctx.state.tables,
            msg.pass_puzzle_id.ok_or(AppError::InvalidRequest)?,
            msg.answer.as_deref().unwrap_or_default(),
        )
        .await?;
    ctx.send_reply(CmdId::ResolvePuzzleCmd, reply, 0, req.up_tag)
        .await
}
