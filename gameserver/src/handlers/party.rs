use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, PartyServerListReply, PartyServerListRequest};

pub async fn on_party_server_list(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    PartyServerListRequest::decode(&req.data[..])?;

    ctx.send_reply(
        CmdId::PartyServerListCmd,
        PartyServerListReply {
            party_servers: vec![],
        },
        0,
        req.up_tag,
    )
    .await
}
