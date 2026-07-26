use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, GetAct229InfoRequest};

pub async fn on_get_act229_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct229InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act229_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct229InfoCmd, reply, 0, req.up_tag)
        .await
}
