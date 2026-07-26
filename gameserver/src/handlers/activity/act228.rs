use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{Act228FlipGridRequest, Act228GetFinalBonusRequest, CmdId, GetAct228InfoRequest};

pub async fn on_get_act228_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetAct228InfoRequest::decode(&req.data[..])?;
    let reply = ctx.player_mut()?.activity.act228_info(msg.activity_id);

    ctx.send_reply(CmdId::GetAct228InfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act228_flip_grid(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act228FlipGridRequest::decode(&req.data[..])?;
    let reply = ctx.player_mut()?.activity.act228_flip_grid(msg.activity_id);

    ctx.send_reply(CmdId::Act228FlipGridCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_act228_get_final_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Act228GetFinalBonusRequest::decode(&req.data[..])?;
    let reply = ctx
        .player_mut()?
        .activity
        .act228_get_final_bonus(msg.activity_id);

    ctx.send_reply(CmdId::Act228GetFinalBonusCmd, reply, 0, req.up_tag)
        .await
}
