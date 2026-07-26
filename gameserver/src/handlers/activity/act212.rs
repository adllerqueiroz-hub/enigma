use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{Act212BonusPush, Act212ReceiveBonusRequest, CmdId, GetAct212InfoRequest};

pub async fn on_get_act212_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct212InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act212_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct212InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act212_receive_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act212ReceiveBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .receive_act212_bonus(db, msg.activity_id, msg.id)
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

    let info = ctx
        .player_mut()?
        .activity
        .act212_info(db, claim.reply.activity_id)
        .await?
        .act212_info;
    ctx.notify(
        CmdId::Act212BonusPushCmd,
        Act212BonusPush { act212_info: info },
    )
    .await?;

    ctx.send_reply(CmdId::Act212ReceiveBonusCmd, claim.reply, 0, req.up_tag)
        .await
}
