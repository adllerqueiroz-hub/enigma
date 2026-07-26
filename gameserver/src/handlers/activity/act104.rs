use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{
    CmdId, Get104InfosRequest, MarkActivity104StoryRequest, MarkEpisodeAfterStoryRequest,
    MarkPopSummaryRequest,
};

pub async fn on_get_104_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = Get104InfosRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .act104_infos(db, msg.activity_id)
        .await?;

    ctx.send_reply(CmdId::Get104InfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mark_episode_after_story(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = MarkEpisodeAfterStoryRequest::decode(&req.data[..])?;
    let activity_id = msg.activity_id.ok_or(AppError::InvalidRequest)?;
    let layer = msg.layer.ok_or(AppError::InvalidRequest)?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .mark_episode_after_story(db, activity_id, layer)
        .await?;

    ctx.send_reply(CmdId::MarkEpisodeAfterStoryCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mark_activity104_story(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = MarkActivity104StoryRequest::decode(&req.data[..])?;
    let activity_id = msg.activity_id.ok_or(AppError::InvalidRequest)?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .mark_activity104_story(db, activity_id)
        .await?;

    ctx.send_reply(CmdId::MarkActivity104StoryCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mark_pop_summary(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = MarkPopSummaryRequest::decode(&req.data[..])?;
    let activity_id = msg.activity_id.ok_or(AppError::InvalidRequest)?;
    let db = ctx.state.db;
    let reply = ctx
        .player_mut()?
        .activity
        .mark_pop_summary(db, activity_id)
        .await?;

    ctx.send_reply(CmdId::MarkPopSummaryCmd, reply, 0, req.up_tag)
        .await
}
