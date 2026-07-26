use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::{material_get_approach::MaterialGetApproach, red_dot_id::RedDotId},
    util::push,
};
use prost::Message;
use sonettobuf::{CmdId, GainRoomHeroFaithRequest, UpdateRoomHeroDataRequest};

pub async fn on_update_room_hero_data(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = UpdateRoomHeroDataRequest::decode(&req.data[..])?;
    let reply = rooms
        .update_room_hero_data(ctx.state.db, ctx.state.tables, &msg.room_hero_ids)
        .await?;
    let (_, changed_info_ids) = ctx
        .player()?
        .red_dot
        .show(ctx.state.db, RedDotId::RoomCharacterFaithFull.id(), false)
        .await?;
    push::send_red_dot_push(
        ctx,
        RedDotId::RoomCharacterFaithFull.id(),
        changed_info_ids,
        true,
    )
    .await?;
    ctx.send_reply(CmdId::UpdateRoomHeroDataCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_gain_room_hero_faith(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let rooms = ctx.player()?.room;
    let msg = GainRoomHeroFaithRequest::decode(&req.data[..])?;
    let gain = rooms
        .gain_room_hero_faith(ctx.state.db, ctx.state.tables, &msg.hero_ids)
        .await?;
    push::send_hero_update_push(ctx, player_id, gain.changed_hero_ids).await?;
    push::send_material_change_push(
        ctx,
        gain.material_changes,
        Some(MaterialGetApproach::RoomGainFaith),
    )
    .await?;
    push::send_red_dot_push(
        ctx,
        RedDotId::RoomCharacterFaithGetFull.id(),
        vec![0],
        false,
    )
    .await?;
    ctx.send_reply(CmdId::GainRoomHeroFaithCmd, gain.reply, 0, req.up_tag)
        .await
}
