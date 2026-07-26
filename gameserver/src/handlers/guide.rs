use crate::{
    error::AppError,
    logic::guide,
    net::{context::ConnectionContext, packet::ClientPacket},
    util::push,
};
use prost::Message;
use sonettobuf::{CmdId, FinishGuideRequest, UpdateGuidePush};

pub async fn on_get_guide_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = guide::get_guide_info(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::GetGuideInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_finish_guide(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = FinishGuideRequest::decode(&req.data[..])?;
    let completion =
        guide::finish_guide(ctx.state.db, player_id, msg.guide_id, msg.step_id).await?;

    ctx.notify(
        CmdId::UpdateGuidePushCmd,
        UpdateGuidePush {
            guide_infos: vec![completion.guide_info],
        },
    )
    .await?;
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        completion.rewards,
        completion.material_changes,
        None,
    )
    .await?;
    if let Some(snapshot) = completion.group_snapshot {
        ctx.notify(CmdId::UpdateHeroGroupSnapshotPushCmd, snapshot)
            .await?;
    }
    ctx.send_reply(CmdId::FinishGuideCmd, completion.reply, 0, req.up_tag)
        .await
}
