use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{Act208ReceiveBonusRequest, CmdId, GetAct208InfoRequest};

pub async fn on_get_act208_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct208InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act208_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct208InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act208_receive_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act208ReceiveBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .receive_act208_bonus(db, msg.activity_id, msg.id)
        .await?;

    if let Some(rewards) = &claim.rewards {
        push::send_item_change_push(
            ctx,
            player_id,
            rewards.item_ids.clone(),
            rewards.power_item_ids.clone(),
            rewards.insight_item_ids.clone(),
        )
        .await?;
        push::send_currency_change_push(ctx, player_id, rewards.currency_ids.clone()).await?;
        push::send_equip_update_push(ctx, player_id, rewards.equip_uids.clone()).await?;
        push::send_hero_update_push(ctx, player_id, rewards.hero_ids.clone()).await?;
        push::send_skin_gain_pushes(
            ctx,
            &rewards.skin_gains,
            Some(MaterialGetApproach::Activity),
        )
        .await?;
        push::send_bp_score_update_pushes(ctx, &rewards.bp_scores).await?;
        push::send_material_change_push(
            ctx,
            claim.material_changes.clone(),
            Some(MaterialGetApproach::Activity),
        )
        .await?;
    }

    ctx.send_reply(CmdId::Act208ReceiveBonusCmd, claim.reply, 0, req.up_tag)
        .await
}
