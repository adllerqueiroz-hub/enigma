use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, ReadBgmRequest, SetFavoriteBgmRequest, SetUseBgmRequest};

pub async fn on_get_bgm_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx.player()?.preferences.bgm_info(ctx.state.db).await?;
    ctx.send_reply(CmdId::GetBgmInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_use_bgm(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let preferences = ctx.player()?.preferences;
    let msg = SetUseBgmRequest::decode(&req.data[..])?;
    let reply = preferences
        .set_use_bgm(ctx.state.db, msg.bgm_id.unwrap_or_default())
        .await?;
    ctx.send_reply(CmdId::SetUseBgmCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_favorite_bgm(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let preferences = ctx.player()?.preferences;
    let msg = SetFavoriteBgmRequest::decode(&req.data[..])?;
    let reply = preferences
        .set_favorite_bgm(
            ctx.state.db,
            msg.bgm_id.unwrap_or_default(),
            msg.favorite.unwrap_or_default(),
        )
        .await?;
    ctx.send_reply(CmdId::SetFavoriteBgmCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_read_bgm(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let preferences = ctx.player()?.preferences;
    let msg = ReadBgmRequest::decode(&req.data[..])?;
    let reply = preferences
        .read_bgm(ctx.state.db, msg.bgm_id.unwrap_or_default())
        .await?;
    ctx.send_reply(CmdId::ReadBgmCmd, reply, 0, req.up_tag)
        .await
}
