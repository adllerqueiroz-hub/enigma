use crate::{
    error::AppError,
    logic::charge,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    types::red_dot_id::RedDotId,
    util::push,
};
use prost::Message;
use sonettobuf::{
    CmdId, GetChargePushInfoReply, GetMonthCardBonusRequest, ReadChargeNewReply,
    ReadChargeNewRequest,
};

pub async fn on_get_charge_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = charge::charge_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetChargeInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_month_card_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = charge::month_card_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetMonthCardInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_month_card_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetMonthCardBonusRequest::decode(&req.data[..])?;
    let claim = charge::month_card_bonus(ctx.state.db, player_id, msg.id).await?;

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
            Some(MaterialGetApproach::MonthCard),
        )
        .await?;
        push::send_bp_score_update_pushes(ctx, &rewards.bp_scores).await?;
        push::send_material_change_push(
            ctx,
            claim.material_changes.clone(),
            Some(MaterialGetApproach::MonthCard),
        )
        .await?;
    }

    ctx.send_reply(CmdId::GetMonthCardBonusCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_charge_push_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    ctx.send_reply(
        CmdId::GetChargePushInfoCmd,
        GetChargePushInfoReply {},
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_read_charge_new(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ReadChargeNewRequest::decode(&req.data[..])?;
    let mut goods_ids = msg.goods_ids.to_vec();
    goods_ids.sort_unstable();
    goods_ids.dedup();

    database::db::game::red_dots::hide_red_dot_infos(
        ctx.state.db,
        player_id,
        RedDotId::StoreChargeGoodsRead.id(),
        goods_ids.clone(),
    )
    .await?;

    ctx.push_red_dot(RedDotId::StoreChargeGoodsRead.id(), vec![0], true)
        .await?;
    ctx.send_reply(
        CmdId::ReadChargeNewCmd,
        ReadChargeNewReply {
            goods_ids: msg.goods_ids,
        },
        0,
        req.up_tag,
    )
    .await
}
