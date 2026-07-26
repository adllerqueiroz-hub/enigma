use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use sonettobuf::CmdId;

pub async fn on_get_explore_simple_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx.player()?.exploration.simple_info(ctx.state.db).await?;
    ctx.send_reply(CmdId::GetExploreSimpleInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_weekwalk_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx
        .player()?
        .exploration
        .weekwalk_info(ctx.state.db)
        .await?;
    ctx.send_reply(CmdId::GetWeekwalkInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_weekwalk_ver2_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx
        .player()?
        .exploration
        .weekwalk_v2_info(ctx.state.db)
        .await?;
    ctx.send_reply(CmdId::WeekwalkVer2GetInfoCmd, reply, 0, req.up_tag)
        .await
}
