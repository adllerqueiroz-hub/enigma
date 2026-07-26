use crate::{
    error::AppError,
    logic::time,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use sonettobuf::CmdId;

pub async fn on_get_server_time(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = time::server_time_reply();
    ctx.send_reply_fixed(CmdId::GetServerTimeCmd, reply, 0, req.up_tag)
        .await
}
