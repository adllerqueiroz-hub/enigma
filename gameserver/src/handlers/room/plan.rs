use super::*;

pub async fn on_get_room_plan_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx
        .player()?
        .room
        .room_plan_info(ctx.state.db, ctx.state.tables)
        .await?;
    ctx.send_reply(CmdId::GetRoomPlanInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_room_plan_details(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = GetRoomPlanDetailsRequest::decode(&req.data[..])?;
    let reply = rooms
        .room_plan_details(ctx.state.db, msg.id.unwrap_or_default())
        .await?;
    ctx.send_reply(CmdId::GetRoomPlanDetailsCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_room_plan(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = SetRoomPlanRequest::decode(&req.data[..])?;
    let reply = rooms
        .set_room_plan(
            ctx.state.db,
            ctx.state.tables,
            msg.id.unwrap_or_default(),
            msg.cover_id.unwrap_or_default(),
            msg.name.unwrap_or_default(),
        )
        .await?;
    ctx.send_reply(CmdId::SetRoomPlanCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_room_plan_name(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = SetRoomPlanNameRequest::decode(&req.data[..])?;
    let reply = rooms
        .set_room_plan_name(
            ctx.state.db,
            msg.id.unwrap_or_default(),
            msg.name.unwrap_or_default(),
        )
        .await?;
    ctx.send_reply(CmdId::SetRoomPlanNameCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_room_plan_cover(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = SetRoomPlanCoverRequest::decode(&req.data[..])?;
    let reply = rooms
        .set_room_plan_cover(
            ctx.state.db,
            msg.id.unwrap_or_default(),
            msg.cover_id.unwrap_or_default(),
        )
        .await?;
    ctx.send_reply(CmdId::SetRoomPlanCoverCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_delete_room_plan(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = DeleteRoomPlanRequest::decode(&req.data[..])?;
    let reply = rooms
        .delete_room_plan(ctx.state.db, msg.id.unwrap_or_default())
        .await?;
    ctx.send_reply(CmdId::DeleteRoomPlanCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_switch_room_plan(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = SwitchRoomPlanRequest::decode(&req.data[..])?;
    let reply = rooms
        .switch_room_plan(
            ctx.state.db,
            ctx.state.tables,
            msg.id_a.unwrap_or_default(),
            msg.id_b.unwrap_or_default(),
        )
        .await?;
    ctx.send_reply(CmdId::SwitchRoomPlanCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_use_room_plan(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = UseRoomPlanRequest::decode(&req.data[..])?;
    let reply = rooms
        .use_room_plan(ctx.state.db, ctx.state.tables, msg.id.unwrap_or_default())
        .await?;
    ctx.send_reply(CmdId::UseRoomPlanCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_share_room_plan(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = ShareRoomPlanRequest::decode(&req.data[..])?;
    let reply = rooms
        .share_room_plan(ctx.state.db, msg.id.unwrap_or_default())
        .await?;
    ctx.send_reply(CmdId::ShareRoomPlanCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_room_share(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetRoomShareRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .room
        .get_room_share(ctx.state.db, msg.share_code.unwrap_or_default())
        .await?;
    ctx.send_reply(CmdId::GetRoomShareCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_use_room_share(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = UseRoomShareRequest::decode(&req.data[..])?;
    let reply = rooms
        .use_room_share(
            ctx.state.db,
            msg.share_code.unwrap_or_default(),
            msg.id.unwrap_or_default(),
            msg.cover_id.unwrap_or_default(),
            msg.name.unwrap_or_default(),
        )
        .await?;
    ctx.send_reply(CmdId::UseRoomShareCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_copy_other_room_plan(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = CopyOtherRoomPlanRequest::decode(&req.data[..])?;
    let reply = rooms
        .copy_other_room_plan(
            ctx.state.db,
            ctx.state.tables,
            msg.target_uid.ok_or(AppError::InvalidRequest)?,
            msg.id.ok_or(AppError::InvalidRequest)?,
            msg.cover_id.unwrap_or_default(),
            msg.name.unwrap_or_default(),
        )
        .await?;
    ctx.send_reply(CmdId::CopyOtherRoomPlanCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_delete_room_share(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = DeleteRoomShareRequest::decode(&req.data[..])?;
    let reply = rooms
        .delete_room_share(ctx.state.db, msg.id.unwrap_or_default())
        .await?;
    ctx.send_reply(CmdId::DeleteRoomShareCmd, reply, 0, req.up_tag)
        .await
}
