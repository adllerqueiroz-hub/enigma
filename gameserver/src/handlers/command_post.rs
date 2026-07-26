use crate::{
    error::AppError,
    logic::command_post,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::{material_get_approach::MaterialGetApproach, red_dot_id::RedDotId},
    util::push,
};
use prost::Message;
use sonettobuf::{
    CmdId, CommandPostBonusAllRequest, CommandPostBonusRequest, CommandPostCharacterReadRequest,
    CommandPostDispatchRequest, CommandPostEventReadRequest, CommandPostPaperRequest,
    FinishCommandPostEventRequest, GetCommandPostInfoRequest,
};

pub async fn on_get_command_post_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    GetCommandPostInfoRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply = command_post::get_command_post_info(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::GetCommandPostInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_command_post_character_read(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = CommandPostCharacterReadRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply = command_post::command_post_character_read(ctx.state.db, player_id, msg.id).await?;

    ctx.send_reply(CmdId::CommandPostCharacterReadCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_command_post_event_read(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = CommandPostEventReadRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply = command_post::command_post_event_read(ctx.state.db, player_id, msg.id).await?;

    ctx.send_reply(CmdId::CommandPostEventReadCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_command_post_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = CommandPostBonusRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let claim = command_post::command_post_bonus(ctx.state.db, player_id, msg.bonus_id).await?;
    push::send_item_first_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::CommandStationPaperReward),
    )
    .await?;
    push::clear_red_dot_infos(ctx, RedDotId::CommandStationBonus.id()).await?;

    ctx.send_reply(CmdId::CommandPostBonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_command_post_bonus_all(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    CommandPostBonusAllRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let claim = command_post::command_post_bonus_all(ctx.state.db, player_id).await?;
    push::send_item_first_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::CommandStationPaperReward),
    )
    .await?;
    push::clear_red_dot_infos(ctx, RedDotId::CommandStationBonus.id()).await?;

    ctx.send_reply(CmdId::CommandPostBonusAllCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_command_post_paper(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    CommandPostPaperRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply = command_post::command_post_paper(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::CommandPostPaperCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_command_post_dispatch(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = CommandPostDispatchRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply =
        command_post::command_post_dispatch(ctx.state.db, player_id, msg.event_id, msg.hero_ids)
            .await?;

    ctx.send_reply(CmdId::CommandPostDispatchCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_finish_command_post_event(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = FinishCommandPostEventRequest::decode(&req.data[..])?;
    let player_id = ctx.player()?.id;
    let reply = command_post::finish_command_post_event(ctx.state.db, player_id, msg.id).await?;

    ctx.send_reply(CmdId::FinishCommandPostEventCmd, reply, 0, req.up_tag)
        .await
}
