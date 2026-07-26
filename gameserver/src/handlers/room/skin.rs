use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, ReadRoomSkinRequest, SetRoomSkinRequest};

pub async fn on_set_room_skin(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = SetRoomSkinRequest::decode(&req.data[..])?;
    let reply = rooms
        .set_room_skin(
            ctx.state.db,
            msg.id.ok_or(AppError::InvalidRequest)?,
            msg.skin_id.ok_or(AppError::InvalidRequest)?,
        )
        .await?;
    ctx.send_reply(CmdId::SetRoomSkinCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_read_room_skin(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = ReadRoomSkinRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .room
        .read_room_skin(msg.skin_id.ok_or(AppError::InvalidRequest)?);
    ctx.send_reply(CmdId::ReadRoomSkinCmd, reply, 0, req.up_tag)
        .await
}
