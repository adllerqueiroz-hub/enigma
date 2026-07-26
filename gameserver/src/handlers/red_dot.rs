use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{CmdId, GetRedDotInfosRequest, ShowRedDotRequest};

pub async fn on_get_red_dot_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetRedDotInfosRequest::decode(&req.data[..])?;
    let reply = ctx.player()?.red_dot.infos(ctx.state.db, msg.ids).await?;

    ctx.send_reply(CmdId::GetRedDotInfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_show_red_dot(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = ShowRedDotRequest::decode(&req.data[..])?;
    let define_id = msg.define_id.ok_or(AppError::InvalidRequest)?;
    let (reply, changed_info_ids) = ctx
        .player()?
        .red_dot
        .show(ctx.state.db, define_id, msg.is_visible.unwrap_or(false))
        .await?;
    ctx.push_red_dot(define_id, changed_info_ids.clone(), false)
        .await?;

    ctx.send_reply(CmdId::ShowRedDotCmd, reply, 0, req.up_tag)
        .await
}
