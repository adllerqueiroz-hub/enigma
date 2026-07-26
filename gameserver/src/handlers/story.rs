use crate::{
    error::AppError,
    logic::story,
    net::{context::ConnectionContext, packet::ClientPacket},
    util::push,
};
use prost::Message;
use sonettobuf::{
    CmdId, FinishNecrologistStoryModeReply, FinishNecrologistStoryModeRequest, GetHeroStoryRequest,
    GetNecrologistStoryRequest, GetStoryFinishRequest, StoryFinishPush,
    UpdateNecrologistStoryRequest, UpdateStoryRequest,
};

pub async fn on_get_story(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = story::get_story(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetStoryCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_story_finish(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetStoryFinishRequest::decode(&req.data[..])?;
    let reply = story::get_story_finish(
        ctx.state.db,
        player_id,
        msg.story_id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::GetStoryFinishCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_update_story(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = UpdateStoryRequest::decode(&req.data[..])?;
    let update = story::update_story(
        ctx.state.db,
        player_id,
        msg.story_id.unwrap_or_default(),
        msg.step_id.unwrap_or_default(),
        msg.favor.unwrap_or_default(),
    )
    .await?;
    if let Some(story_id) = update.finished_story_id {
        ctx.notify(
            CmdId::StoryFinishPushCmd,
            StoryFinishPush {
                story_id: Some(story_id),
            },
        )
        .await?;
        push::send_dungeon_map_progression(ctx, player_id).await?;
    }
    ctx.send_reply(CmdId::UpdateStoryCmd, update.reply, 0, req.up_tag)
        .await
}

pub async fn on_get_hero_story(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    GetHeroStoryRequest::decode(&req.data[..])?;
    let reply = story::get_hero_story(ctx.state.db, player_id).await?;
    ctx.send_reply(CmdId::GetHeroStoryCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_necrologist_story(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetNecrologistStoryRequest::decode(&req.data[..])?;
    let reply = story::get_necrologist_story(
        ctx.state.db,
        player_id,
        msg.story_id.unwrap_or_default(),
        ctx.state.tables,
    )
    .await?;
    ctx.send_reply(CmdId::GetNecrologistStoryCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_update_necrologist_story(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = UpdateNecrologistStoryRequest::decode(&req.data[..])?;
    let reply = story::update_necrologist_story(
        ctx.state.db,
        player_id,
        msg.story_id.unwrap_or_default(),
        msg.info.unwrap_or_else(|| "{}".to_string()),
        msg.plot_infos,
    )
    .await?;
    ctx.send_reply(CmdId::UpdateNecrologistStoryCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_finish_necrologist_story_mode(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = FinishNecrologistStoryModeRequest::decode(&req.data[..])?;
    let reply = FinishNecrologistStoryModeReply {
        story_id: msg.story_id,
        mode_id: msg.mode_id,
    };
    ctx.send_reply(CmdId::FinishNecrologistStoryModeCmd, reply, 0, req.up_tag)
        .await
}
