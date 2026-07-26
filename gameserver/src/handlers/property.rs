use crate::{
    error::AppError,
    logic::property,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, SetMainSceneSkinRequest, SetSimplePropertyRequest, SetUiStyleSkinRequest};

pub async fn on_get_simple_property(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = property::get_simple_property(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::GetSimplePropertyCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_simple_property(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetSimplePropertyRequest::decode(&req.data[..])?;
    let property_id = msg.id.ok_or(AppError::InvalidRequest)?;
    let property_value = msg.property.ok_or(AppError::InvalidRequest)?;

    let (reply, push) =
        property::set_simple_property(ctx.state.db, player_id, property_id, property_value).await?;

    ctx.notify(CmdId::SimplePropertyPushCmd, push).await?;
    ctx.send_reply(CmdId::SetSimplePropertyCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_main_scene_skin(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetMainSceneSkinRequest::decode(&req.data[..])?;
    let reply = property::set_main_scene_skin(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.item_id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::SetMainSceneSkinCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_ui_style_skin(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetUiStyleSkinRequest::decode(&req.data[..])?;
    let reply = property::set_ui_style_skin(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.item_id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::SetUiStyleSkinCmd, reply, 0, req.up_tag)
        .await
}
