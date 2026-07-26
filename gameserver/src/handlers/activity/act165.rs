use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{
    Act165GainMilestoneRewardRequest, Act165GenerateEndingRequest, Act165GetInfoRequest,
    Act165ModifyKeywordRequest, Act165RestartRequest, CmdId,
};

pub async fn on_act165_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act165GetInfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act165_get_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Act165GetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act165_modify_keyword(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act165ModifyKeywordRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act165_modify_keyword(db, msg.activity_id, msg.story_id, msg.keyword_ids)
        .await?;

    ctx.send_reply(CmdId::Act165ModifyKeywordCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act165_generate_ending(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act165GenerateEndingRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act165_generate_ending(db, msg.activity_id, msg.story_id)
        .await?;

    ctx.send_reply(CmdId::Act165GenerateEndingCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act165_restart(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act165RestartRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act165_restart(db, msg.activity_id, msg.story_id, msg.step_id)
        .await?;

    ctx.send_reply(CmdId::Act165RestartCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act165_gain_milestone_reward(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act165GainMilestoneRewardRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act165_gain_milestone_reward(db, msg.activity_id, msg.story_id)
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

    ctx.send_reply(
        CmdId::Act165GainMilestoneRewardCmd,
        claim.reply,
        0,
        req.up_tag,
    )
    .await
}
