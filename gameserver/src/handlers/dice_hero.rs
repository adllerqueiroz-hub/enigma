use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{
    CmdId, DiceHeroEnterStoryRequest, DiceHeroGetInfoRequest, DiceHeroGetRewardRequest,
};
pub async fn on_dice_hero_get_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    DiceHeroGetInfoRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .activity
        .dice_hero_info(ctx.state.db, ctx.state.tables)
        .await?;
    ctx.send_reply(CmdId::DiceHeroGetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_dice_hero_enter_story(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = DiceHeroEnterStoryRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .activity
        .dice_hero_enter_story(
            ctx.state.db,
            msg.chapter.unwrap_or_default(),
            msg.level_id.unwrap_or_default(),
            ctx.state.tables,
        )
        .await?;
    ctx.send_reply(CmdId::DiceHeroEnterStoryCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_dice_hero_get_reward(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = DiceHeroGetRewardRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .activity
        .dice_hero_get_reward(
            ctx.state.db,
            msg.chapter.unwrap_or_default(),
            msg.index,
            ctx.state.tables,
        )
        .await?;
    ctx.send_reply(CmdId::DiceHeroGetRewardCmd, reply, 0, req.up_tag)
        .await
}
