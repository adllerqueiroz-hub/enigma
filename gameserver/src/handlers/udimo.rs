use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, GetUdimoInfoRequest};

pub async fn on_get_info(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    GetUdimoInfoRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .udimo
        .info(ctx.state.db, ctx.state.tables)
        .await?;
    ctx.send_reply(CmdId::GetUdimoInfoCmd, reply, 0, req.up_tag)
        .await
}
