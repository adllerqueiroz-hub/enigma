use crate::{
    error::AppError,
    logic::misc,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, GetPowerMakerInfoRequest, HandbookReadRequest, UpdateSettingInfoRequest};

pub async fn on_get_handbook_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = misc::handbook_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetHandbookInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_handbook_read(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = HandbookReadRequest::decode(&req.data[..])?;
    let reply = misc::handbook_read(
        ctx.state.db,
        player_id,
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
    let player_id = ctx.player()?.id;
    let reply = misc::setting_infos(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetSettingInfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_update_setting_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = UpdateSettingInfoRequest::decode(&req.data[..])?;
    let reply = misc::update_setting_info(
        ctx.state.db,
        player_id,
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
    let player_id = ctx.player()?.id;
    let reply = misc::unlock_voucher_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetUnlockVoucherInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_power_maker_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let request = GetPowerMakerInfoRequest::decode(&req.data[..])?;
    let reply = misc::power_maker_info(
        ctx.state.db,
        player_id,
        request.is_login.unwrap_or_default(),
    )
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
