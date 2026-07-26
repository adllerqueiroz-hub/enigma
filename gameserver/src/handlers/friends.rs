use crate::{
    error::AppError,
    logic::friends,
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
    let player_id = ctx.player()?.id;
    LoadFriendInfosRequest::decode(&req.data[..])?;
    let reply = friends::load_friend_infos(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::LoadFriendInfosCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_friend_info_list(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    GetFriendInfoListRequest::decode(&req.data[..])?;
    let reply = friends::get_friend_info_list(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::GetFriendInfoListCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_recommended_friends(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    GetRecommendedFriendsRequest::decode(&req.data[..])?;
    let reply = friends::get_recommended_friends(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::GetRecommendedFriendsCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_apply_list(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    ctx.player()?;
    GetApplyListRequest::decode(&req.data[..])?;
    let reply = friends::get_apply_list();

    ctx.send_reply(CmdId::GetApplyListCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_blacklist(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    GetBlacklistRequest::decode(&req.data[..])?;
    let reply = friends::get_blacklist(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::GetBlacklistCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_remove_friend(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = RemoveFriendRequest::decode(&req.data[..])?;
    let friend_id = msg.friend_id.ok_or(AppError::InvalidRequest)?;
    let reply = friends::remove_friend(ctx.state.db, player_id, friend_id).await?;

    ctx.send_reply(CmdId::RemoveFriendCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_add_blacklist(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = AddBlacklistRequest::decode(&req.data[..])?;
    let friend_id = msg.friend_id.ok_or(AppError::InvalidRequest)?;
    let reply = friends::add_blacklist(ctx.state.db, player_id, friend_id).await?;

    ctx.send_reply(CmdId::AddBlacklistCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_remove_blacklist(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = RemoveBlacklistRequest::decode(&req.data[..])?;
    let friend_id = msg.friend_id.ok_or(AppError::InvalidRequest)?;
    let reply = friends::remove_blacklist(ctx.state.db, player_id, friend_id).await?;

    ctx.send_reply(CmdId::RemoveBlacklistCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_search(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SearchRequest::decode(&req.data[..])?;
    let value = msg.value.ok_or(AppError::InvalidRequest)?;
    let reply = friends::search(ctx.state.db, player_id, value).await?;

    ctx.send_reply(CmdId::SearchCmd, reply, 0, req.up_tag).await
}
