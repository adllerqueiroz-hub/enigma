use crate::{
    error::AppError,
    logic::equipment,
    net::{context::ConnectionContext, packet::ClientPacket},
    util::{push, task_events},
};
use database::db::game::tasks::TaskEvent;
use prost::Message;
use sonettobuf::{
    CmdId, EquipBreakRequest, EquipDecomposeRequest, EquipDeletePush, EquipLockRequest,
    EquipRefineRequest, EquipStrengthenRequest,
};

pub async fn on_get_equip_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = equipment::equip_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetEquipInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_equip_lock(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = EquipLockRequest::decode(&req.data[..])?;
    let target_uid = msg.target_uid.ok_or(AppError::InvalidRequest)?;
    let lock = msg.lock.ok_or(AppError::InvalidRequest)?;
    let reply = equipment::equip_lock(ctx.state.db, player_id, target_uid, lock).await?;

    ctx.send_reply(CmdId::EquipLockCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_equip_strengthen(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = EquipStrengthenRequest::decode(&req.data[..])?;
    let target_uid = msg.target_uid.ok_or(AppError::InvalidRequest)?;
    let (reply, changed_uids) =
        equipment::strengthen(ctx.state.db, player_id, target_uid, msg.eat_equips).await?;

    push::send_equip_update_push(ctx, player_id, changed_uids).await?;
    task_events::notify(
        ctx,
        player_id,
        TaskEvent::DoneCount {
            name: "EquipStrengthen",
            count: 1,
        },
    )
    .await?;
    ctx.send_reply(CmdId::EquipStrengthenCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_equip_break(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = EquipBreakRequest::decode(&req.data[..])?;
    let target_uid = msg.target_uid.ok_or(AppError::InvalidRequest)?;
    let (reply, changed_currencies, changed_items, changed_uids) =
        equipment::break_equip(ctx.state.db, player_id, target_uid).await?;

    push::send_currency_change_push(ctx, player_id, changed_currencies).await?;
    push::send_item_change_push(ctx, player_id, changed_items, Vec::new(), Vec::new()).await?;
    push::send_equip_update_push(ctx, player_id, changed_uids).await?;
    ctx.send_reply(CmdId::EquipBreakCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_equip_refine(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = EquipRefineRequest::decode(&req.data[..])?;
    let target_uid = msg.target_uid.ok_or(AppError::InvalidRequest)?;
    let (reply, changed_uids, delete_uids) =
        equipment::refine(ctx.state.db, player_id, target_uid, msg.eat_uids).await?;

    if !delete_uids.is_empty() {
        ctx.notify(
            CmdId::EquipDeletePushCmd,
            EquipDeletePush { uids: delete_uids },
        )
        .await?;
    }
    push::send_equip_update_push(ctx, player_id, changed_uids).await?;
    ctx.send_reply(CmdId::EquipRefineCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_equip_decompose(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = EquipDecomposeRequest::decode(&req.data[..])?;
    let (reply, changed_uids) =
        equipment::decompose(ctx.state.db, player_id, msg.equip_uids.clone()).await?;

    ctx.notify(
        CmdId::EquipDeletePushCmd,
        EquipDeletePush {
            uids: msg.equip_uids,
        },
    )
    .await?;
    push::send_equip_update_push(ctx, player_id, changed_uids).await?;
    ctx.send_reply(CmdId::EquipDecomposeCmd, reply, 0, req.up_tag)
        .await
}
