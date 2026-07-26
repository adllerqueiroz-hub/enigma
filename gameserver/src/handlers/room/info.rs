use super::*;

pub async fn on_get_room_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx.player()?.room.room_info(ctx.state.db).await?;
    ctx.send_reply(CmdId::GetRoomInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_room_ob_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = GetRoomObInfoRequest::decode(&req.data[..])?;
    let reply = rooms
        .room_ob_info(ctx.state.db, msg.need_block_data.unwrap_or_default())
        .await?;
    ctx.send_reply(CmdId::GetRoomObInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_room_log(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx.player()?.room.room_log(ctx.state.db).await?;
    ctx.send_reply(CmdId::GetRoomLogCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_other_room_ob_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetOtherRoomObInfoRequest::decode(&req.data[..])?;
    let target_uid = msg.target_uid.unwrap_or(ctx.player()?.id);
    let reply = ctx
        .player()?
        .room
        .other_room_ob_info(ctx.state.db, target_uid)
        .await?;
    ctx.send_reply(CmdId::GetOtherRoomObInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_read_room_log_new(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = ReadRoomLogNewRequest::decode(&req.data[..])?;
    let reply = rooms.read_room_log_new(ctx.state.db, msg.index).await?;
    ctx.send_reply(CmdId::ReadRoomLogNewCmd, reply, 0, req.up_tag)
        .await
}
