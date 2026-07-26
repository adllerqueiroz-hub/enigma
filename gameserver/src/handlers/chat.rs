use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, DeleteOfflineMsgRequest, ReportRequest, SendMsgRequest, WordTestRequest};

pub async fn on_send_msg(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let social = ctx.player()?.social;
    let msg = SendMsgRequest::decode(&req.data[..])?;
    let sequence = ctx.next_sequence();
    let (push, reply) = social.send_msg(ctx.state.db, sequence, msg).await?;

    ctx.notify(CmdId::ChatMsgPushCmd, push).await?;
    ctx.send_reply(CmdId::SendMsgCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_delete_offline_msg(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let social = ctx.player()?.social;
    DeleteOfflineMsgRequest::decode(&req.data[..])?;
    ctx.send_reply(
        CmdId::DeleteOfflineMsgCmd,
        social.delete_offline_msg(),
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_report(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let social = ctx.player()?.social;
    ReportRequest::decode(&req.data[..])?;
    ctx.send_reply(CmdId::ReportCmd, social.report(), 0, req.up_tag)
        .await
}

pub async fn on_word_test(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let social = ctx.player()?.social;
    WordTestRequest::decode(&req.data[..])?;
    ctx.send_reply(CmdId::WordTestCmd, social.word_test(), 0, req.up_tag)
        .await
}
