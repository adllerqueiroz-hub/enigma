use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{Act197ExploreRequest, Act197RummageRequest, CmdId};

pub async fn on_get_197_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get197InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act197_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get197InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act197_rummage(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act197RummageRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act197_rummage(db, msg.activity_id, msg.pool_id)
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    ctx.send_reply(CmdId::Act197RummageCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_act197_explore(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act197ExploreRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let result = ctx
        .player_mut()?
        .activity
        .act197_explore(db, msg.activity_id, msg.r#type)
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        result.rewards,
        result.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    ctx.send_reply(CmdId::Act197ExploreCmd, result.reply, 0, req.up_tag)
        .await
}
