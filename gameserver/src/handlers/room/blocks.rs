use super::*;

pub async fn on_get_block_package_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx.player()?.room.block_package_info(ctx.state.db).await?;
    ctx.send_reply(CmdId::GetBlockPackageInfoRequsetCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_hide_block_package_reddot(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = HideBlockPackageReddotRequest::decode(&req.data[..])?;
    let reply = rooms
        .hide_block_package_reddot(ctx.state.db, msg.id.ok_or(AppError::InvalidRequest)?)
        .await?;
    ctx.send_reply(CmdId::HideBlockPackageReddotCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_hide_building_reddot(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = HideBuildingReddotRequset::decode(&req.data[..])?;
    let reply = rooms
        .hide_building_reddot(ctx.state.db, msg.id.ok_or(AppError::InvalidRequest)?)
        .await?;
    ctx.send_reply(CmdId::HideBuildingReddotRequsetCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_block_permanent_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = GetBlockPermanentInfoRequest::decode(&req.data[..])?;
    let reply = rooms
        .block_permanent_info(ctx.state.db, ctx.state.tables, msg.block_ids)
        .await?;
    ctx.send_reply(CmdId::GetBlockPermanentInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_building_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx.player()?.room.building_info(ctx.state.db).await?;
    ctx.send_reply(CmdId::GetBuildingInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_use_block(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = UseBlockRequest::decode(&req.data[..])?;
    let reply = rooms
        .use_block(
            ctx.state.db,
            msg.block_id.ok_or(AppError::InvalidRequest)?,
            msg.block_package_id.unwrap_or_default(),
            msg.rotate.unwrap_or_default(),
            msg.x.unwrap_or_default(),
            msg.y.unwrap_or_default(),
        )
        .await?;
    ctx.send_reply(CmdId::UseBlockCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_unuse_block(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = UnUseBlockRequest::decode(&req.data[..])?;
    let reply = rooms.unuse_blocks(ctx.state.db, msg.block_ids).await?;
    ctx.send_reply(CmdId::UnUseBlockCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_reset_room(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let reply = ctx.player()?.room.reset_room(ctx.state.db).await?;
    ctx.send_reply(CmdId::ResetRoomCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_water_type(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = SetWaterTypeRequest::decode(&req.data[..])?;
    let changes: Vec<_> = msg
        .water_infos
        .into_iter()
        .map(|info| {
            (
                info.block_id.unwrap_or_default(),
                info.water_type.unwrap_or_default(),
            )
        })
        .collect();
    let reply = rooms.set_water_types(ctx.state.db, &changes).await?;
    ctx.send_reply(CmdId::SetWaterTypeCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_block_color(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = SetBlockColorRequest::decode(&req.data[..])?;
    let changes: Vec<_> = msg
        .block_color_infos
        .into_iter()
        .map(|info| {
            (
                info.block_id.unwrap_or_default(),
                info.block_color.unwrap_or_default(),
            )
        })
        .collect();
    let reply = rooms.set_block_colors(ctx.state.db, &changes).await?;
    ctx.send_reply(CmdId::SetBlockColorCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_use_building(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = UseBuildingRequest::decode(&req.data[..])?;
    let reply = rooms
        .use_building(
            ctx.state.db,
            msg.uid.unwrap_or_default(),
            msg.x.unwrap_or_default(),
            msg.y.unwrap_or_default(),
            msg.rotate.unwrap_or_default(),
        )
        .await?;
    ctx.send_reply(CmdId::UseBuildingCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_unuse_building(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = UnUseBuildingRequest::decode(&req.data[..])?;
    let reply = rooms
        .unuse_building(ctx.state.db, msg.uid.unwrap_or_default())
        .await?;
    ctx.send_reply(CmdId::UnUseBuildingCmd, reply, 0, req.up_tag)
        .await
}
