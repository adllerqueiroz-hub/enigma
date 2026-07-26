use crate::{
    error::AppError,
    logic::odyssey,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, OdysseyGetInfoRequest};

pub async fn on_odyssey_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let _ = OdysseyGetInfoRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply = odyssey::get_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::OdysseyGetInfoCmd, reply, 0, req.up_tag)
        .await
}
