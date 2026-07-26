use crate::{
    error::AppError,
    logic::stat,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{
    ClientStatBaseInfoRequest, CmdId, GpuCpuLogRequest, UpdateClientStatBaseInfoRequest,
};

pub async fn on_client_stat_base_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    ClientStatBaseInfoRequest::decode(&req.data[..])?;
    let push = stat::base_info(ctx.state.db, player_id).await?;

    ctx.notify(CmdId::StatInfoPushCmd, push).await?;
    ctx.send_empty_reply(CmdId::ClientStatBaseInfoCmd, Vec::new(), 0, req.up_tag)
        .await
}

pub async fn on_update_client_stat_base_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    UpdateClientStatBaseInfoRequest::decode(&req.data[..])?;
    let reply = stat::update_base_info(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::UpdateClientStatBaseInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_gpu_cpu_log(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GpuCpuLogRequest::decode(&req.data[..])?;
    tracing::debug!(player_id, cpu = ?msg.cpu, gpu = ?msg.gpu, "client hardware report");

    ctx.send_empty_reply(CmdId::GpuCpuLogCmd, Vec::new(), 0, req.up_tag)
        .await
}
