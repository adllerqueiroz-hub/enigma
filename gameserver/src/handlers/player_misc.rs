use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use sonettobuf::CmdId;

pub async fn on_get_assist_bonus(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx.player()?.social.assist_bonus();

    ctx.send_reply(CmdId::GetAssistBonusCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_cloth_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx.player()?.profile.cloth_info(ctx.state.db).await?;

    ctx.send_reply(CmdId::GetClothInfoCmd, reply, 0, req.up_tag)
        .await
}
