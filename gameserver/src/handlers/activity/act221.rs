use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{Act221SelectRequest, Act221SummonRequest, CmdId};

pub async fn on_get_221_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Get221InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act221_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get221InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act221_summon(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act221SummonRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act221_summon(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Act221SummonCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act221_select(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act221SelectRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act221_select(db, msg.activity_id, msg.select)
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    ctx.send_reply(CmdId::Act221SelectCmd, claim.reply, 0, req.up_tag)
        .await
}
