use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, DebugLogoutReply, DebugLogoutRequest, LogoutReply, LogoutRequest};

pub async fn on_logout(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    LogoutRequest::decode(&req.data[..])?;
    ctx.send_reply(CmdId::LogoutCmd, LogoutReply {}, 0, req.up_tag)
        .await?;
    ctx.request_disconnect();
    Ok(())
}

pub async fn on_debug_logout(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    DebugLogoutRequest::decode(&req.data[..])?;
    ctx.send_reply(CmdId::DebugLogoutCmd, DebugLogoutReply {}, 0, req.up_tag)
        .await?;
    ctx.request_disconnect();
    Ok(())
}
