use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{Act160FinishMissionRequest, Act160GetInfoRequest, Act160UpdatePush, CmdId};

pub async fn on_act160_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act160GetInfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act160_get_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Act160GetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act160_finish_mission(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act160FinishMissionRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .finish_act160_mission(db, msg.activity_id, msg.id)
        .await?;

    push::send_item_change_push(
        ctx,
        player_id,
        claim.rewards.item_ids.clone(),
        claim.rewards.power_item_ids.clone(),
        claim.rewards.insight_item_ids.clone(),
    )
    .await?;
    push::send_currency_change_push(ctx, player_id, claim.rewards.currency_ids.clone()).await?;
    push::send_equip_update_push(ctx, player_id, claim.rewards.equip_uids.clone()).await?;
    push::send_hero_update_push(ctx, player_id, claim.rewards.hero_ids.clone()).await?;
    push::send_skin_gain_pushes(
        ctx,
        &claim.rewards.skin_gains,
        Some(MaterialGetApproach::Activity),
    )
    .await?;
    push::send_bp_score_update_pushes(ctx, &claim.rewards.bp_scores).await?;
    push::send_material_change_push(
        ctx,
        claim.material_changes.clone(),
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    for act160_info in claim.updates.iter().skip(1) {
        ctx.notify(
            CmdId::Act160UpdatePushCmd,
            Act160UpdatePush {
                activity_id: claim.reply.activity_id,
                act160_info: Some(*act160_info),
            },
        )
        .await?;
    }

    ctx.send_reply(CmdId::Act160FinishMissionCmd, claim.reply, 0, req.up_tag)
        .await
}
