use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, OdysseyGetInfoRequest};

pub async fn on_odyssey_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let _ = OdysseyGetInfoRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .odyssey
        .info(ctx.state.db, ctx.state.tables)
        .await?;
    ctx.send_reply(CmdId::OdysseyGetInfoCmd, reply, 0, req.up_tag)
        .await
}
