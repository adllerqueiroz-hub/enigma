use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::push,
};
use prost::Message;
use sonettobuf::{
    Act216TaskPush, CmdId, FinishAct216TaskRequest, GetAct216InfoRequest, GetAct216OnceBonusRequest,
};

pub async fn on_get_act216_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct216InfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act216_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::GetAct216InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_finish_act216_task(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = FinishAct216TaskRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .finish_act216_task(db, msg.activity_id, msg.task_id)
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
    ctx.notify(
        CmdId::Act216TaskPushCmd,
        Act216TaskPush {
            activity_id: claim.reply.activity_id,
            act216_tasks: vec![claim.task_info],
            delete_tasks: Vec::new(),
        },
    )
    .await?;

    ctx.send_reply(CmdId::FinishAct216TaskCmd, claim.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_act216_once_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetAct216OnceBonusRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .get_act216_once_bonus(db, msg.activity_id)
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

    ctx.send_reply(CmdId::GetAct216OnceBonusCmd, claim.reply, 0, req.up_tag)
        .await
}
