use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::{push, task_events},
};
use logic::task::TaskEvent;
use prost::Message;
use sonettobuf::{CmdId, FinishAct125EpisodeRequest, GetAct125InfosRequest};

pub async fn on_get_act125_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct125InfosRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act125_infos(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct125InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_finish_act125_episode(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = FinishAct125EpisodeRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let episode_id = msg.episode_id.unwrap_or_default();
    let reply = ctx
        .player_mut()?
        .activity
        .finish_act125_episode(db, msg.activity_id, msg.episode_id, msg.target_frequency)
        .await?;

    task_events::notify(ctx, player_id, TaskEvent::EpisodeFinish { episode_id }).await?;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        reply.rewards,
        reply.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;
    ctx.send_reply(CmdId::FinishAct125EpisodeCmd, reply.reply, 0, req.up_tag)
        .await
}
