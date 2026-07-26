use crate::{
    error::AppError,
    logic::rouge,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{
    CmdId, GetRouge2InfoRequest, GetRouge2OutsideInfoRequest, GetRougeOutsideInfoRequest,
    Rouge2GetUnlockCollectionsRequest,
};

pub async fn on_get_rouge_outside_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetRougeOutsideInfoRequest::decode(&req.data[..])?;
    let reply = rouge::rouge_outside_info(msg.season.unwrap_or_default());
    ctx.send_reply(CmdId::GetRougeOutsideInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_rouge2_outside_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let _ = GetRouge2OutsideInfoRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply = rouge::rouge2_outside_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetRouge2OutsideInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_rouge2_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let _ = GetRouge2InfoRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply = rouge::rouge2_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetRouge2InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_rouge2_get_unlock_collections(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let _ = Rouge2GetUnlockCollectionsRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply = rouge::rouge2_unlock_collections(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::Rouge2GetUnlockCollectionsCmd, reply, 0, req.up_tag)
        .await
}
