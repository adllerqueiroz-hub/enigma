use crate::{
    error::AppError,
    logic::survival,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, SurvivalOutSideGetInfoRequest};

pub async fn on_get_outside_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    SurvivalOutSideGetInfoRequest::decode(&req.data[..])?;
    ctx.send_reply(
        CmdId::SurvivalOutSideGetInfoCmd,
        survival::outside_info(ctx.state.tables),
        0,
        req.up_tag,
    )
    .await
}
