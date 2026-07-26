use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, GetPowerMakerInfoRequest, HandbookReadRequest, UpdateSettingInfoRequest};

pub async fn on_get_handbook_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx.player()?.collection.handbook_info(ctx.state.db).await?;
    ctx.send_reply(CmdId::GetHandbookInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_handbook_read(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = HandbookReadRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .collection
        .read_handbook(
            ctx.state.db,
            msg.r#type.ok_or(AppError::InvalidRequest)?,
            msg.id.ok_or(AppError::InvalidRequest)?,
        )
        .await?;
    ctx.send_reply(CmdId::HandbookReadCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_setting_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx
        .player()?
        .preferences
        .setting_infos(ctx.state.db)
        .await?;
    ctx.send_reply(CmdId::GetSettingInfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_update_setting_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let preferences = ctx.player()?.preferences;
    let msg = UpdateSettingInfoRequest::decode(&req.data[..])?;
    let reply = preferences
        .update_setting(
            ctx.state.db,
            msg.r#type.unwrap_or_default(),
            msg.param.unwrap_or_default(),
        )
        .await?;
    ctx.send_reply(CmdId::UpdateSettingInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_unlock_voucher_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx
        .player()?
        .inventory
        .unlock_voucher_info(ctx.state.db)
        .await?;
    ctx.send_reply(CmdId::GetUnlockVoucherInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_power_maker_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = GetPowerMakerInfoRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .inventory
        .power_maker_info(ctx.state.db, request.is_login.unwrap_or_default())
        .await?;
    ctx.send_reply(CmdId::GetPowerMakerInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mark_main_thumbnail(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    ctx.send_empty_reply(CmdId::MarkMainThumbnailCmd, Vec::new(), 0, req.up_tag)
        .await
}
