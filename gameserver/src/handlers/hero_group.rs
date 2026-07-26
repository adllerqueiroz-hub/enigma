use crate::{
    error::AppError,
    logic::hero_group,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{
    ChangeHeroGroupSelectRequest, CheckHeroGroupNameRequest, CmdId, DeleteHeroGroupRequest,
    GetHeroGroupSnapshotListRequest, SetHeroGroupEquipRequest, SetHeroGroupSnapshotRequest,
    UpdateHeroGroupNameRequest, UpdateHeroGroupRequest, UpdateHeroGroupSortRequest,
};

pub async fn on_check_hero_group_name(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    ctx.player()?;
    let msg = CheckHeroGroupNameRequest::decode(&req.data[..])?;
    let reply = hero_group::check_name(msg.name.as_deref().unwrap_or_default())?;

    ctx.send_reply(CmdId::CheckHeroGroupNameCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_update_hero_group_name(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = UpdateHeroGroupNameRequest::decode(&req.data[..])?;
    let reply = hero_group::update_name(
        ctx.state.db,
        player_id,
        msg.id.ok_or(AppError::InvalidRequest)?,
        msg.current_select.ok_or(AppError::InvalidRequest)?,
        msg.name.ok_or(AppError::InvalidRequest)?,
    )
    .await?;

    ctx.send_reply(CmdId::UpdateHeroGroupNameCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_update_hero_group_sort(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = UpdateHeroGroupSortRequest::decode(&req.data[..])?;
    let reply = hero_group::update_sort(
        ctx.state.db,
        player_id,
        msg.snapshot_id.ok_or(AppError::InvalidRequest)?,
        msg.sort_sub_ids,
    )
    .await?;
    ctx.send_reply(CmdId::UpdateHeroGroupSortCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_delete_hero_group(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = DeleteHeroGroupRequest::decode(&req.data[..])?;
    let reply = hero_group::delete_group(
        ctx.state.db,
        player_id,
        msg.snapshot_id.ok_or(AppError::InvalidRequest)?,
        msg.snapshot_sub_id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::DeleteHeroGroupCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_update_hero_group(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = UpdateHeroGroupRequest::decode(&req.data[..])?;
    let reply = hero_group::update_group(ctx.state.db, player_id, msg.group_info).await?;
    ctx.send_reply(CmdId::HeroGroupUpdateHeroGroupCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_hero_group_common_list(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = hero_group::common_list(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetHeroGroupCommonListCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_hero_group_list(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = hero_group::current_list(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetHeroGroupListCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_hero_group_snapshot_list(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetHeroGroupSnapshotListRequest::decode(&req.data[..])?;
    let reply = hero_group::snapshot_list(ctx.state.db, player_id, msg.snapshot_id).await?;
    ctx.send_reply(CmdId::GetHeroGroupSnapshotListCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_hero_group_equip(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetHeroGroupEquipRequest::decode(&req.data[..])?;
    let group_id = msg.group_id.ok_or(AppError::InvalidRequest)?;
    let equip = msg.equip.ok_or(AppError::InvalidRequest)?;
    let reply = hero_group::set_equip(ctx.state.db, player_id, group_id, equip).await?;

    ctx.send_reply(CmdId::SetHeroGroupEquipCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_hero_group_snapshot(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetHeroGroupSnapshotRequest::decode(&req.data[..])?;
    let snapshot_id = msg.snapshot_id.ok_or(AppError::InvalidRequest)?;
    let snapshot_sub_id = msg.snapshot_sub_id.unwrap_or_default();
    let fight_group = msg.fight_group.ok_or(AppError::InvalidRequest)?;
    let reply = hero_group::set_snapshot(
        ctx.state.db,
        player_id,
        snapshot_id,
        snapshot_sub_id,
        fight_group,
    )
    .await?;

    ctx.send_reply(CmdId::SetHeroGroupSnapshotCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_change_hero_group_select(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ChangeHeroGroupSelectRequest::decode(&req.data[..])?;
    let reply = hero_group::change_selection(
        ctx.state.db,
        player_id,
        msg.id.ok_or(AppError::InvalidRequest)?,
        msg.current_select.ok_or(AppError::InvalidRequest)?,
    )
    .await?;

    ctx.send_reply(CmdId::ChangeHeroGroupSelectCmd, reply, 0, req.up_tag)
        .await
}
