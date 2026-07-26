use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{Act218AcceptRewardRequest, Act218FinishGameRequest, CmdId, Get218InfoRequest};

pub async fn on_get_218_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Get218InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act218_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get218InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act218_finish_game(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act218FinishGameRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .finish_act218_game(db, msg.activity_id, msg.result, msg.game_record)
        .await?;

    ctx.send_reply(CmdId::Act218FinishGameCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act218_accept_reward(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act218AcceptRewardRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .accept_act218_reward(db, msg.activity_id)
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    ctx.send_reply(CmdId::Act218AcceptRewardCmd, claim.reply, 0, req.up_tag)
        .await
}
