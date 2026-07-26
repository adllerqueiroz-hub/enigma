use crate::{
    error::AppError,
    logic::bgm,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, ReadBgmRequest, SetFavoriteBgmRequest, SetUseBgmRequest};

pub async fn on_get_bgm_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = bgm::bgm_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetBgmInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_use_bgm(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetUseBgmRequest::decode(&req.data[..])?;
    let reply = bgm::set_use_bgm(ctx.state.db, player_id, msg.bgm_id.unwrap_or_default()).await?;
    ctx.send_reply(CmdId::SetUseBgmCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_favorite_bgm(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetFavoriteBgmRequest::decode(&req.data[..])?;
    let reply = bgm::set_favorite_bgm(
        ctx.state.db,
        player_id,
        msg.bgm_id.unwrap_or_default(),
        msg.favorite.unwrap_or_default(),
    )
    .await?;
    ctx.send_reply(CmdId::SetFavoriteBgmCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_read_bgm(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ReadBgmRequest::decode(&req.data[..])?;
    let reply = bgm::read_bgm(ctx.state.db, player_id, msg.bgm_id.unwrap_or_default()).await?;
    ctx.send_reply(CmdId::ReadBgmCmd, reply, 0, req.up_tag)
        .await
}
