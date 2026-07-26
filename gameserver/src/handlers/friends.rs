use crate::{
    error::AppError,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{
    AddBlacklistRequest, CmdId, GetApplyListRequest, GetBlacklistRequest, GetFriendInfoListRequest,
    GetRecommendedFriendsRequest, LoadFriendInfosRequest, RemoveBlacklistRequest,
    RemoveFriendRequest, SearchRequest,
};

pub async fn on_load_friend_infos(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let social = ctx.player()?.social;
    LoadFriendInfosRequest::decode(&req.data[..])?;
    let reply = social.load_friend_infos(ctx.state.db).await?;

    ctx.send_reply(CmdId::LoadFriendInfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_friend_info_list(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let social = ctx.player()?.social;
    GetFriendInfoListRequest::decode(&req.data[..])?;
    let reply = social.get_friend_info_list(ctx.state.db).await?;

    ctx.send_reply(CmdId::GetFriendInfoListCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_recommended_friends(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let social = ctx.player()?.social;
    GetRecommendedFriendsRequest::decode(&req.data[..])?;
    let reply = social.get_recommended_friends(ctx.state.db).await?;

    ctx.send_reply(CmdId::GetRecommendedFriendsCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_apply_list(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let social = ctx.player()?.social;
    GetApplyListRequest::decode(&req.data[..])?;
    let reply = social.get_apply_list();

    ctx.send_reply(CmdId::GetApplyListCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_blacklist(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let social = ctx.player()?.social;
    GetBlacklistRequest::decode(&req.data[..])?;
    let reply = social.get_blacklist(ctx.state.db).await?;

    ctx.send_reply(CmdId::GetBlacklistCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_remove_friend(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let social = ctx.player()?.social;
    let msg = RemoveFriendRequest::decode(&req.data[..])?;
    let friend_id = msg.friend_id.ok_or(AppError::InvalidRequest)?;
    let reply = social.remove_friend(ctx.state.db, friend_id).await?;

    ctx.send_reply(CmdId::RemoveFriendCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_add_blacklist(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let social = ctx.player()?.social;
    let msg = AddBlacklistRequest::decode(&req.data[..])?;
    let friend_id = msg.friend_id.ok_or(AppError::InvalidRequest)?;
    let reply = social.add_blacklist(ctx.state.db, friend_id).await?;

    ctx.send_reply(CmdId::AddBlacklistCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_remove_blacklist(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let social = ctx.player()?.social;
    let msg = RemoveBlacklistRequest::decode(&req.data[..])?;
    let friend_id = msg.friend_id.ok_or(AppError::InvalidRequest)?;
    let reply = social.remove_blacklist(ctx.state.db, friend_id).await?;

    ctx.send_reply(CmdId::RemoveBlacklistCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_search(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let social = ctx.player()?.social;
    let msg = SearchRequest::decode(&req.data[..])?;
    let value = msg.value.ok_or(AppError::InvalidRequest)?;
    let reply = social.search(ctx.state.db, value).await?;

    ctx.send_reply(CmdId::SearchCmd, reply, 0, req.up_tag).await
}
