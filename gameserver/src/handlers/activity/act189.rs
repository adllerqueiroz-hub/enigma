use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{CmdId, GetAct189InfoRequest, GetAct189OnceBonusRequest};

pub async fn on_get_act189_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct189InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act189_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct189InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act189_once_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetAct189OnceBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .get_act189_once_bonus(db, msg.activity_id)
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

    ctx.send_reply(CmdId::GetAct189OnceBonusCmd, claim.reply, 0, req.up_tag)
        .await
}
