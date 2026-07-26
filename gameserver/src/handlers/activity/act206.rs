use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{Act206ChooseDirectionRequest, Act206GetBonusRequest, CmdId};

pub async fn on_act206_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Act206GetInfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act206_get_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Act206GetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act206_choose_direction(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act206ChooseDirectionRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act206_choose_direction(db, msg.activity_id, msg.direction_id)
        .await?;

    ctx.send_reply(CmdId::Act206ChooseDirectionCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act206_get_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act206GetBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act206_get_bonus(db, msg.activity_id)
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    ctx.send_reply(CmdId::Act206GetBonusCmd, claim.reply, 0, req.up_tag)
        .await
}
