use super::*;

pub async fn on_generate_road(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = GenerateRoadRequest::decode(&req.data[..])?;
    let reply = rooms
        .generate_roads(ctx.state.db, msg.ids, msg.road_infos)
        .await?;
    ctx.send_reply(CmdId::GenerateRoadCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_delete_road(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = DeleteRoadRequest::decode(&req.data[..])?;
    let reply = rooms.delete_roads(ctx.state.db, msg.ids).await?;
    ctx.send_reply(CmdId::DeleteRoadCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_allot_critter(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = AllotCritterRequest::decode(&req.data[..])?;
    let reply = rooms
        .allot_road_critter(
            ctx.state.db,
            msg.id.unwrap_or_default(),
            msg.critter_uid.unwrap_or_default(),
        )
        .await?;
    ctx.send_reply(CmdId::AllotCritterCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_allot_vehicle(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = AllotVehicleRequest::decode(&req.data[..])?;
    let reply = rooms
        .allot_road_vehicle(
            ctx.state.db,
            msg.id.unwrap_or_default(),
            msg.building_uid.unwrap_or_default(),
            msg.skin_id.unwrap_or_default(),
        )
        .await?;
    ctx.send_reply(CmdId::AllotVehicleCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_room_confirm(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let reply = rooms.room_confirm(ctx.state.db).await?;
    let tasks = rooms
        .sync_room_tasks(ctx.state.db, ctx.state.tables)
        .await?;
    task_events::notify_tasks(ctx, tasks).await?;
    ctx.send_reply(CmdId::RoomConfirmCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_room_revert(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx.player()?.room.room_revert(ctx.state.db).await?;
    ctx.send_reply(CmdId::RoomRevertCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_room_theme_collection_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let rooms = ctx.player()?.room;
    let msg = GetRoomThemeCollectionBonusRequest::decode(&req.data[..])?;
    let outcome = rooms
        .room_theme_collection_bonus(
            ctx.state.db,
            ctx.state.tables,
            msg.id.ok_or(AppError::InvalidRequest)?,
        )
        .await?;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        outcome.rewards,
        outcome.material_changes,
        None,
    )
    .await?;
    ctx.send_reply(
        CmdId::GetRoomThemeCollectionBonusCmd,
        outcome.reply,
        0,
        req.up_tag,
    )
    .await
}
