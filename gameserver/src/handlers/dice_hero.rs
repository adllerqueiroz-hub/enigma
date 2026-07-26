use crate::{
    error::AppError,
    logic::dice_hero,
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
    let player_id = ctx.player()?.id;
    DiceHeroGetInfoRequest::decode(&req.data[..])?;
    let reply = dice_hero::dice_hero_info(ctx.state.db, player_id, ctx.state.tables).await?;
    ctx.send_reply(CmdId::DiceHeroGetInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_dice_hero_enter_story(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = DiceHeroEnterStoryRequest::decode(&req.data[..])?;
    let reply = dice_hero::dice_hero_enter_story(
        ctx.state.db,
        player_id,
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
    let player_id = ctx.player()?.id;
    let msg = DiceHeroGetRewardRequest::decode(&req.data[..])?;
    let reply = dice_hero::dice_hero_get_reward(
        ctx.state.db,
        player_id,
        msg.chapter.unwrap_or_default(),
        msg.index,
        ctx.state.tables,
    )
    .await?;
    ctx.send_reply(CmdId::DiceHeroGetRewardCmd, reply, 0, req.up_tag)
        .await
}
