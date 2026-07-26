use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{Act198GainRequest, CmdId};

pub async fn on_act198_gain(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act198GainRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act198_gain(db, msg.activity_id)
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

    ctx.send_reply(CmdId::Act198GainCmd, claim.reply, 0, req.up_tag)
        .await
}
