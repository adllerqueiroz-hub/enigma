use crate::{
    error::AppError,
    logic::chat,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, DeleteOfflineMsgRequest, ReportRequest, SendMsgRequest, WordTestRequest};

pub async fn on_send_msg(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SendMsgRequest::decode(&req.data[..])?;
    let sequence = ctx.next_sequence();
    let (push, reply) = chat::send_msg(ctx.state.db, player_id, sequence, msg).await?;

    ctx.notify(CmdId::ChatMsgPushCmd, push).await?;
    ctx.send_reply(CmdId::SendMsgCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_delete_offline_msg(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    ctx.player()?;
    DeleteOfflineMsgRequest::decode(&req.data[..])?;
    ctx.send_reply(
        CmdId::DeleteOfflineMsgCmd,
        chat::delete_offline_msg(),
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_report(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    ctx.player()?;
    ReportRequest::decode(&req.data[..])?;
    ctx.send_reply(CmdId::ReportCmd, chat::report(), 0, req.up_tag)
        .await
}

pub async fn on_word_test(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    ctx.player()?;
    WordTestRequest::decode(&req.data[..])?;
    ctx.send_reply(CmdId::WordTestCmd, chat::word_test(), 0, req.up_tag)
        .await
}
