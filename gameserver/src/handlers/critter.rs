use crate::{
    error::AppError,
    logic::critter,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{
    ChangeRestCritterRequest, CmdId, CritterRenameRequest, GetCritterBookInfoRequest,
    LockCritterRequest, MarkCritterBookNewReadRequest, SetCritterBookBackgroundRequest,
    SetCritterBookUseSpecialSkinRequest,
};
pub async fn on_critter_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = critter::critter_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::CritterGetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_critter_rename(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = CritterRenameRequest::decode(&req.data[..])?;
    let reply = critter::critter_rename(
        ctx.state.db,
        player_id,
        msg.uid.unwrap_or_default(),
        msg.name.unwrap_or_default(),
    )
    .await?;
    ctx.send_reply(CmdId::CritterRenameCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_lock_critter(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = LockCritterRequest::decode(&req.data[..])?;
    let reply = critter::lock_critter(
        ctx.state.db,
        player_id,
        msg.uid.unwrap_or_default(),
        msg.lock.unwrap_or_default(),
    )
    .await?;
    ctx.send_reply(CmdId::LockCritterCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_change_rest_critter(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ChangeRestCritterRequest::decode(&req.data[..])?;
    let reply = critter::change_rest_critter(
        ctx.state.db,
        player_id,
        msg.building_uid.unwrap_or_default(),
        msg.operation.unwrap_or_default(),
        msg.slot_id1.unwrap_or_default(),
        msg.critter_uid.unwrap_or_default(),
        msg.slot_id2.unwrap_or_default(),
    )
    .await?;
    ctx.send_reply(CmdId::ChangeRestCritterCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_book_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    GetCritterBookInfoRequest::decode(&req.data[..])?;
    let reply = critter::get_book_info(ctx.state.db, ctx.state.tables, ctx.player()?.id).await?;
    ctx.send_reply(CmdId::GetCritterBookInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mark_book_read(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = MarkCritterBookNewReadRequest::decode(&req.data[..])?;
    let reply = critter::mark_book_read(
        ctx.state.db,
        ctx.state.tables,
        ctx.player()?.id,
        msg.id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::MarkCritterBookNewReadCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_book_background(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = SetCritterBookBackgroundRequest::decode(&req.data[..])?;
    let reply = critter::set_book_background(
        ctx.state.db,
        ctx.state.tables,
        ctx.player()?.id,
        msg.id.ok_or(AppError::InvalidRequest)?,
        msg.background.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::SetCritterBookBackgroundCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_book_special_skin(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = SetCritterBookUseSpecialSkinRequest::decode(&req.data[..])?;
    let reply = critter::set_book_special_skin(
        ctx.state.db,
        ctx.state.tables,
        ctx.player()?.id,
        msg.id.ok_or(AppError::InvalidRequest)?,
        msg.use_special_skin.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::SetCritterBookUseSpecialSkinCmd, reply, 0, req.up_tag)
        .await
}
