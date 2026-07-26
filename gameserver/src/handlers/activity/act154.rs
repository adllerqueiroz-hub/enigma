use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{Answer154PuzzleRequest, CmdId, Get154InfosRequest};

pub async fn on_get_154_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Get154InfosRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act154_infos(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get154InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_answer154_puzzle(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Answer154PuzzleRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .answer154_puzzle(db, msg.activity_id, msg.puzzle_id, msg.option_id)
        .await?;

    if let Some(rewards) = claim.rewards {
        push::send_applied_reward_pushes(
            ctx,
            player_id,
            rewards,
            claim.material_changes,
            Some(MaterialGetApproach::Activity),
        )
        .await?;
    }

    ctx.send_reply(CmdId::Answer154PuzzleCmd, claim.reply, 0, req.up_tag)
        .await
}
