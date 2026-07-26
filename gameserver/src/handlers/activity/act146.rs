use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{
    Act146EpisodeBonusRequest, CmdId, FinishAct146EpisodeRequest, GetAct146InfosRequest,
};

pub async fn on_get_act146_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct146InfosRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act146_infos(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct146InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_finish_act146_episode(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = FinishAct146EpisodeRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .finish_act146_episode(db, msg.activity_id, msg.episode_id)
        .await?;

    ctx.send_reply(CmdId::FinishAct146EpisodeCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act146_episode_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act146EpisodeBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act146_episode_bonus(db, msg.activity_id, msg.episode_id)
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    ctx.send_reply(CmdId::Act146EpisodeBonusCmd, claim.reply, 0, req.up_tag)
        .await
}
