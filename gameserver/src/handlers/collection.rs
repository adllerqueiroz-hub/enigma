use crate::{
    error::AppError,
    logic::collection,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{
    CmdId, ReadNewAchievementRequest, RecordDialogInfoRequest, ShowAchievementRequest,
    UpdateRedDotPush,
};

pub async fn on_get_achievement_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let db = ctx.state.db;
    let reply = ctx.player_mut()?.collection.achievement_info(db).await?;
    ctx.send_reply(CmdId::GetAchievementInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_read_new_achievement(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = ReadNewAchievementRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let (reply, update) = ctx
        .player_mut()?
        .collection
        .read_new_achievement(db, msg.ids)
        .await?;

    ctx.notify(CmdId::UpdateAchievementPushCmd, update).await?;
    let red_dot_infos = ctx
        .player()?
        .red_dot
        .infos(
            ctx.state.db,
            vec![crate::types::red_dot_id::RedDotId::AchievementFinish.id()],
        )
        .await?
        .red_dot_infos;
    ctx.notify(
        CmdId::UpdateRedDotPushCmd,
        UpdateRedDotPush {
            red_dot_infos,
            replace_all: None,
        },
    )
    .await?;

    ctx.send_reply(CmdId::ReadNewAchievementCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_show_achievement(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ShowAchievementRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .collection
        .show_achievement(db, msg.ids, msg.group_id)
        .await?;

    ctx.notify(
        CmdId::PlayerInfoPushCmd,
        crate::logic::player_info::snapshot(ctx.state.db, player_id).await?,
    )
    .await?;

    ctx.send_reply(CmdId::ShowAchievementCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_antique_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = collection::antique_info(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetAntiqueInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_dialog_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let db = ctx.state.db;
    let reply = ctx.player_mut()?.collection.dialog_info(db).await?;
    ctx.send_reply(CmdId::GetDialogInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_record_dialog_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = RecordDialogInfoRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .collection
        .record_dialog(db, msg.dialog_id)
        .await?;

    ctx.send_reply(CmdId::RecordDialogInfoCmd, reply, 0, req.up_tag)
        .await
}
