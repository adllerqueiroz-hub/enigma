use crate::{
    error::AppError,
    logic::{critter, player_info, session},
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, CritterInfoPush, RenameRequest, UpdateTaskPush};

pub async fn on_login(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let login = session::parse_login_request(&req.data)?;
    let db = ctx.state.db;

    let session = match session::validate_login(db, login).await {
        Ok(session) => session,
        Err(err) => {
            let payload = session::login_error_payload(&err.to_string());
            ctx.send_raw_reply_fixed(CmdId::LoginCmd, payload, 1, req.up_tag)
                .await?;
            return Ok(());
        }
    };

    let updated_tasks = session::start_session(ctx, session).await?;
    let payload = session::login_reply_payload(session.user_id);
    ctx.send_raw_reply_fixed(CmdId::LoginCmd, payload, 0, req.up_tag)
        .await?;

    ctx.register();
    let critter_infos = critter::critter_info(ctx.state.db, session.user_id)
        .await?
        .critter_infos;
    ctx.notify(CmdId::CritterInfoPushCmd, CritterInfoPush { critter_infos })
        .await?;
    if !updated_tasks.is_empty() {
        ctx.notify(
            CmdId::UpdateTaskPushCmd,
            UpdateTaskPush {
                task_info: updated_tasks.into_iter().map(Into::into).collect(),
                activity_info: Vec::new(),
            },
        )
        .await?;
    }
    Ok(())
}

pub async fn on_reconnect(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    ctx.send_empty_reply(CmdId::ReconnectCmd, vec![0x01], 0, req.up_tag)
        .await
}

pub async fn on_rename(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = RenameRequest::decode(&req.data[..])?;
    let name = msg.name.unwrap_or_default();
    let guide_id = msg.guide_id.unwrap_or(1);
    let step_id = msg.step_id.unwrap_or(-1);
    let (reply, push) =
        player_info::rename(ctx.state.db, player_id, name, guide_id, step_id).await?;

    ctx.notify(CmdId::PlayerInfoPushCmd, push).await?;
    ctx.send_reply(CmdId::RenameCmd, reply, 0, req.up_tag).await
}
