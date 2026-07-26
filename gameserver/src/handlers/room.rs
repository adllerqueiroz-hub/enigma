use crate::{
    error::AppError,
    logic::room,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::{material_get_approach::MaterialGetApproach, red_dot_id::RedDotId},
    util::{push, task_events},
};
use database::db::game::{
    open_infos,
    tasks::{ProductionLineAction, TaskEvent},
};
use prost::Message;
use sonettobuf::{
    AllotCritterRequest, AllotVehicleRequest, CmdId, CopyOtherRoomPlanRequest, DeleteRoadRequest,
    DeleteRoomPlanRequest, DeleteRoomShareRequest, GainProductionLineRequest,
    GainRoomHeroFaithRequest, GenerateRoadRequest, GetBlockPermanentInfoRequest,
    GetCharacterInteractionBonusRequest, GetOtherRoomObInfoRequest, GetRoomObInfoRequest,
    GetRoomPlanDetailsRequest, GetRoomShareRequest, GetRoomThemeCollectionBonusRequest,
    HideBlockPackageReddotRequest, HideBuildingReddotRequset, ProductionLineAccelerateRequest,
    ProductionLineInfoRequest, ProductionLineLvUpRequest, ReadRoomLogNewRequest,
    ReadRoomSkinRequest, SetBlockColorRequest, SetRoomPlanCoverRequest, SetRoomPlanNameRequest,
    SetRoomPlanRequest, SetRoomSkinRequest, SetWaterTypeRequest, ShareRoomPlanRequest,
    StartCharacterInteractionRequest, StartProductionLineRequest, SwitchRoomPlanRequest,
    UnUseBlockRequest, UnUseBuildingRequest, UpdateOpenPush, UpdateRoomHeroDataRequest,
    UseBlockRequest, UseBuildingRequest, UseRoomPlanRequest, UseRoomShareRequest,
};

pub async fn on_get_block_package_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = room::block_package_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetBlockPackageInfoRequsetCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_hide_block_package_reddot(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = HideBlockPackageReddotRequest::decode(&req.data[..])?;
    let reply = room::hide_block_package_reddot(
        ctx.state.db,
        player_id,
        msg.id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::HideBlockPackageReddotCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_hide_building_reddot(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = HideBuildingReddotRequset::decode(&req.data[..])?;
    let reply = room::hide_building_reddot(
        ctx.state.db,
        player_id,
        msg.id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::HideBuildingReddotRequsetCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_block_permanent_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetBlockPermanentInfoRequest::decode(&req.data[..])?;
    let reply =
        room::block_permanent_info(ctx.state.db, ctx.state.tables, player_id, msg.block_ids)
            .await?;
    ctx.send_reply(CmdId::GetBlockPermanentInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_building_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = room::building_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetBuildingInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_use_block(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = UseBlockRequest::decode(&req.data[..])?;
    let reply = room::use_block(
        ctx.state.db,
        player_id,
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
    let player_id = ctx.player()?.id;
    let msg = UnUseBlockRequest::decode(&req.data[..])?;
    let reply = room::unuse_blocks(ctx.state.db, player_id, msg.block_ids).await?;
    ctx.send_reply(CmdId::UnUseBlockCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_reset_room(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = room::reset_room(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::ResetRoomCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_water_type(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
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
    let reply = room::set_water_types(ctx.state.db, player_id, &changes).await?;
    ctx.send_reply(CmdId::SetWaterTypeCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_block_color(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
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
    let reply = room::set_block_colors(ctx.state.db, player_id, &changes).await?;
    ctx.send_reply(CmdId::SetBlockColorCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_use_building(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = UseBuildingRequest::decode(&req.data[..])?;
    let reply = room::use_building(
        ctx.state.db,
        player_id,
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
    let player_id = ctx.player()?.id;
    let msg = UnUseBuildingRequest::decode(&req.data[..])?;
    let reply = room::unuse_building(ctx.state.db, player_id, msg.uid.unwrap_or_default()).await?;
    ctx.send_reply(CmdId::UnUseBuildingCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_character_interaction_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = room::character_interaction_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetCharacterInteractionInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_start_character_interaction(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = StartCharacterInteractionRequest::decode(&req.data[..])?;
    let reply = room::start_character_interaction(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::StartCharacterInteractionCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_character_interaction_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetCharacterInteractionBonusRequest::decode(&req.data[..])?;
    let outcome = room::complete_character_interaction(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.id.ok_or(AppError::InvalidRequest)?,
        msg.select_ids,
    )
    .await?;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        outcome.rewards,
        outcome.material_changes,
        Some(MaterialGetApproach::RoomInteraction),
    )
    .await?;
    ctx.send_reply(
        CmdId::GetCharacterInteractionBonusCmd,
        outcome.reply,
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_get_room_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = room::room_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetRoomInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_generate_road(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GenerateRoadRequest::decode(&req.data[..])?;
    let reply = room::generate_roads(ctx.state.db, player_id, msg.ids, msg.road_infos).await?;
    ctx.send_reply(CmdId::GenerateRoadCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_delete_road(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = DeleteRoadRequest::decode(&req.data[..])?;
    let reply = room::delete_roads(ctx.state.db, player_id, msg.ids).await?;
    ctx.send_reply(CmdId::DeleteRoadCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_allot_critter(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = AllotCritterRequest::decode(&req.data[..])?;
    let reply = room::allot_road_critter(
        ctx.state.db,
        player_id,
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
    let player_id = ctx.player()?.id;
    let msg = AllotVehicleRequest::decode(&req.data[..])?;
    let reply = room::allot_road_vehicle(
        ctx.state.db,
        player_id,
        msg.id.unwrap_or_default(),
        msg.building_uid.unwrap_or_default(),
        msg.skin_id.unwrap_or_default(),
    )
    .await?;
    ctx.send_reply(CmdId::AllotVehicleCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_room_ob_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetRoomObInfoRequest::decode(&req.data[..])?;
    let reply = room::room_ob_info(
        ctx.state.db,
        player_id,
        msg.need_block_data.unwrap_or_default(),
    )
    .await?;
    ctx.send_reply(CmdId::GetRoomObInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_room_plan_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = room::room_plan_info(ctx.state.db, ctx.state.tables, player_id).await?;
    ctx.send_reply(CmdId::GetRoomPlanInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_room_plan_details(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetRoomPlanDetailsRequest::decode(&req.data[..])?;
    let reply =
        room::room_plan_details(ctx.state.db, player_id, msg.id.unwrap_or_default()).await?;
    ctx.send_reply(CmdId::GetRoomPlanDetailsCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_room_plan(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetRoomPlanRequest::decode(&req.data[..])?;
    let reply = room::set_room_plan(
        ctx.state.db,
        ctx.state.tables,
        player_id,
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
    let player_id = ctx.player()?.id;
    let msg = SetRoomPlanNameRequest::decode(&req.data[..])?;
    let reply = room::set_room_plan_name(
        ctx.state.db,
        player_id,
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
    let player_id = ctx.player()?.id;
    let msg = SetRoomPlanCoverRequest::decode(&req.data[..])?;
    let reply = room::set_room_plan_cover(
        ctx.state.db,
        player_id,
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
    let player_id = ctx.player()?.id;
    let msg = DeleteRoomPlanRequest::decode(&req.data[..])?;
    let reply = room::delete_room_plan(ctx.state.db, player_id, msg.id.unwrap_or_default()).await?;
    ctx.send_reply(CmdId::DeleteRoomPlanCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_switch_room_plan(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SwitchRoomPlanRequest::decode(&req.data[..])?;
    let reply = room::switch_room_plan(
        ctx.state.db,
        ctx.state.tables,
        player_id,
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
    let player_id = ctx.player()?.id;
    let msg = UseRoomPlanRequest::decode(&req.data[..])?;
    let reply = room::use_room_plan(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.id.unwrap_or_default(),
    )
    .await?;
    ctx.send_reply(CmdId::UseRoomPlanCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_share_room_plan(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ShareRoomPlanRequest::decode(&req.data[..])?;
    let reply = room::share_room_plan(ctx.state.db, player_id, msg.id.unwrap_or_default()).await?;
    ctx.send_reply(CmdId::ShareRoomPlanCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_room_share(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetRoomShareRequest::decode(&req.data[..])?;
    let reply = room::get_room_share(ctx.state.db, msg.share_code.unwrap_or_default()).await?;
    ctx.send_reply(CmdId::GetRoomShareCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_use_room_share(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = UseRoomShareRequest::decode(&req.data[..])?;
    let reply = room::use_room_share(
        ctx.state.db,
        player_id,
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
    let player_id = ctx.player()?.id;
    let msg = CopyOtherRoomPlanRequest::decode(&req.data[..])?;
    let reply = room::copy_other_room_plan(
        ctx.state.db,
        ctx.state.tables,
        player_id,
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
    let player_id = ctx.player()?.id;
    let msg = DeleteRoomShareRequest::decode(&req.data[..])?;
    let reply =
        room::delete_room_share(ctx.state.db, player_id, msg.id.unwrap_or_default()).await?;
    ctx.send_reply(CmdId::DeleteRoomShareCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_room_log(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = room::room_log(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetRoomLogCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_room_confirm(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = room::room_confirm(ctx.state.db, player_id).await?;
    let tasks = room::sync_room_tasks(ctx.state.db, ctx.state.tables, player_id).await?;
    task_events::notify_tasks(ctx, tasks).await?;
    ctx.send_reply(CmdId::RoomConfirmCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_room_revert(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = room::room_revert(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::RoomRevertCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_other_room_ob_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetOtherRoomObInfoRequest::decode(&req.data[..])?;
    let target_uid = msg.target_uid.unwrap_or(ctx.player()?.id);
    let reply = room::other_room_ob_info(ctx.state.db, target_uid).await?;
    ctx.send_reply(CmdId::GetOtherRoomObInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_read_room_log_new(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ReadRoomLogNewRequest::decode(&req.data[..])?;
    let reply = room::read_room_log_new(ctx.state.db, player_id, msg.index).await?;
    ctx.send_reply(CmdId::ReadRoomLogNewCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_room_theme_collection_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetRoomThemeCollectionBonusRequest::decode(&req.data[..])?;
    let outcome = room::room_theme_collection_bonus(
        ctx.state.db,
        ctx.state.tables,
        player_id,
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

pub async fn on_production_line_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ProductionLineInfoRequest::decode(&req.data[..])?;
    let reply = room::production_line_info(ctx.state.db, player_id, &msg.ids).await?;
    ctx.send_reply(CmdId::ProductionLineInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_start_production_line(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = StartProductionLineRequest::decode(&req.data[..])?;
    let line_id = msg.id.ok_or(AppError::InvalidRequest)?;
    let formula_id = msg
        .formula_produce
        .first()
        .and_then(|formula| formula.formula_id);
    let count = msg
        .formula_produce
        .first()
        .and_then(|formula| formula.count)
        .unwrap_or(1);
    let outcome = room::start_production_line(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        line_id,
        formula_id,
        count,
    )
    .await?;
    push::send_cost_pushes(
        ctx,
        player_id,
        outcome.consumed_item_ids,
        outcome.consumed_currency_ids,
        outcome.material_changes,
    )
    .await?;
    task_events::notify(
        ctx,
        player_id,
        TaskEvent::ProductionLine {
            action: ProductionLineAction::Create,
            count,
        },
    )
    .await?;
    ctx.send_reply(CmdId::StartProductionLineCmd, outcome.reply, 0, req.up_tag)
        .await
}

pub async fn on_gain_production_line(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GainProductionLineRequest::decode(&req.data[..])?;
    let outcome =
        room::gain_production_line(ctx.state.db, ctx.state.tables, player_id, &msg.ids).await?;
    let gathered_count = outcome.reply.production_lines.len() as i32;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        outcome.rewards,
        outcome.material_changes,
        Some(MaterialGetApproach::RoomProductLine),
    )
    .await?;
    if gathered_count > 0 {
        task_events::notify(
            ctx,
            player_id,
            TaskEvent::ProductionLine {
                action: ProductionLineAction::Gather,
                count: gathered_count,
            },
        )
        .await?;
    }
    ctx.send_reply(CmdId::GainProductionLineCmd, outcome.reply, 0, req.up_tag)
        .await
}

pub async fn on_production_line_lv_up(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ProductionLineLvUpRequest::decode(&req.data[..])?;
    let reply = room::production_line_lv_up(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.id.ok_or(AppError::InvalidRequest)?,
        msg.new_level.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    push::send_cost_pushes(
        ctx,
        player_id,
        reply.consumed_item_ids,
        reply.consumed_currency_ids,
        reply.material_changes,
    )
    .await?;
    ctx.send_reply(CmdId::ProductionLineLvUpCmd, reply.reply, 0, req.up_tag)
        .await
}

pub async fn on_room_level_up(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = room::room_level_up(ctx.state.db, ctx.state.tables, player_id).await?;
    push::send_cost_pushes(
        ctx,
        player_id,
        reply.consumed_item_ids,
        reply.consumed_currency_ids,
        reply.material_changes,
    )
    .await?;
    let open_infos = open_infos::reconcile_progression(ctx.state.db, player_id).await?;
    if !open_infos.is_empty() {
        ctx.notify(CmdId::UpdateOpenPushCmd, UpdateOpenPush { open_infos })
            .await?;
    }
    let tasks = room::sync_room_tasks(ctx.state.db, ctx.state.tables, player_id).await?;
    task_events::notify_tasks(ctx, tasks).await?;
    ctx.send_reply(CmdId::RoomLevelUpCmd, reply.reply, 0, req.up_tag)
        .await
}

pub async fn on_production_line_accelerate(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ProductionLineAccelerateRequest::decode(&req.data[..])?;
    let reply = room::production_line_accelerate(
        ctx.state.db,
        player_id,
        msg.id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::ProductionLineAccelerateCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_room_skin(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetRoomSkinRequest::decode(&req.data[..])?;
    let reply = room::set_room_skin(
        ctx.state.db,
        player_id,
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
    let reply = room::read_room_skin(msg.skin_id.ok_or(AppError::InvalidRequest)?);
    ctx.send_reply(CmdId::ReadRoomSkinCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_update_room_hero_data(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = UpdateRoomHeroDataRequest::decode(&req.data[..])?;
    let reply = room::update_room_hero_data(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        &msg.room_hero_ids,
    )
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
    let msg = GainRoomHeroFaithRequest::decode(&req.data[..])?;
    let gain = room::gain_room_hero_faith(ctx.state.db, ctx.state.tables, player_id, &msg.hero_ids)
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
