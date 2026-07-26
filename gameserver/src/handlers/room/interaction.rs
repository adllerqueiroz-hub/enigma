use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{CmdId, GetCharacterInteractionBonusRequest, StartCharacterInteractionRequest};

pub async fn on_get_character_interaction_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx
        .player()?
        .room
        .character_interaction_info(ctx.state.db)
        .await?;
    ctx.send_reply(CmdId::GetCharacterInteractionInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_start_character_interaction(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let rooms = ctx.player()?.room;
    let msg = StartCharacterInteractionRequest::decode(&req.data[..])?;
    let reply = rooms
        .start_character_interaction(
            ctx.state.db,
            ctx.state.tables,
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
    let rooms = ctx.player()?.room;
    let msg = GetCharacterInteractionBonusRequest::decode(&req.data[..])?;
    let outcome = rooms
        .complete_character_interaction(
            ctx.state.db,
            ctx.state.tables,
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
