use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
    types::material_get_approach::MaterialGetApproach,
    util::{push, task_events},
};
use logic::task::TaskEvent;
use prost::Message;
use sonettobuf::{Act205FinishGameRequest, Act205GetGameInfoRequest, CmdId};

pub async fn on_act205_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = sonettobuf::Act205GetInfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act205_get_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Act205GetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act205_get_game_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act205GetGameInfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act205_get_game_info(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Act205GetGameInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act205_finish_game(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = Act205FinishGameRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let claim = ctx
        .player_mut()?
        .activity
        .act205_finish_game(
            db,
            msg.activity_id,
            msg.game_type,
            msg.game_info,
            msg.reward_id,
        )
        .await?;

    push::send_applied_reward_pushes(
        ctx,
        player_id,
        claim.rewards,
        claim.material_changes,
        Some(MaterialGetApproach::Activity),
    )
    .await?;

    task_events::notify(
        ctx,
        player_id,
        TaskEvent::Act205FinishGame {
            activity_id: claim.activity_id,
            game_type: claim.game_type,
            is_win: claim.is_win,
        },
    )
    .await?;

    ctx.send_reply(CmdId::Act205FinishGameCmd, claim.reply, 0, req.up_tag)
        .await
}
