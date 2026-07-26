use crate::{
    error::AppError,
    logic::tower_compose,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, TowerComposeGetInfoRequest, TowerComposeSetModsRequest};

pub async fn on_tower_compose_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    TowerComposeGetInfoRequest::decode(&req.data[..])?;
    let reply = tower_compose::get_info(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::TowerComposeGetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_tower_compose_set_mods(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = TowerComposeSetModsRequest::decode(&req.data[..])?;
    let reply = tower_compose::set_mods(
        ctx.state.db,
        player_id,
        msg.theme_id.unwrap_or_default(),
        msg.plane_mods,
    )
    .await?;

    ctx.send_reply(CmdId::TowerComposeSetModsCmd, reply, 0, req.up_tag)
        .await
}
