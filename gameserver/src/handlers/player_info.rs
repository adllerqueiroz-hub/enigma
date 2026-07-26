use crate::{
    error::AppError,
    logic::player_info,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{
    CmdId, GetOpenInfoRequest, GetOtherPlayerInfoRequest, SetBirthdayRequest,
    SetCharacterAgeRequest, SetPlayerBgRequest, SetPortraitRequest, SetShowHeroUniqueIdsRequest,
    SetSignatureRequest,
};

pub async fn on_get_player_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = player_info::get_player_info(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::GetPlayerInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_other_player_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetOtherPlayerInfoRequest::decode(&req.data[..])?;
    let reply = player_info::get_other_player_info(
        ctx.state.db,
        msg.user_id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;

    ctx.send_reply(CmdId::GetOtherPlayerInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_open_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetOpenInfoRequest::decode(&req.data[..])?;
    let reply = player_info::get_open_info(ctx.state.db, player_id, msg.id).await?;

    ctx.send_reply(CmdId::GetOpenInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_hero_info_list(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = player_info::hero_info_list(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::HeroInfoListCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_portrait(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetPortraitRequest::decode(&req.data[..])?;
    let portrait = msg.portrait.ok_or(AppError::InvalidRequest)?;
    let reply = player_info::set_portrait(ctx.state.db, player_id, portrait).await?;

    ctx.send_reply(CmdId::SetPortraitCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_signature(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetSignatureRequest::decode(&req.data[..])?;
    let reply = player_info::set_signature(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.signature.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::SetSignatureCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_birthday(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetBirthdayRequest::decode(&req.data[..])?;
    let reply = player_info::set_birthday(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.birthday.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::SetBirthdayCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_character_age(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetCharacterAgeRequest::decode(&req.data[..])?;
    let reply = player_info::set_character_age(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.character_age,
    )
    .await?;
    ctx.send_reply(CmdId::SetCharacterAgeCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_player_bg(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetPlayerBgRequest::decode(&req.data[..])?;
    let reply = player_info::set_player_bg(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.bg_id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::SetPlayerBgCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_show_hero_unique_ids(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetShowHeroUniqueIdsRequest::decode(&req.data[..])?;
    let (reply, push) =
        player_info::set_show_hero_unique_ids(ctx.state.db, player_id, msg.show_hero_unique_ids)
            .await?;

    ctx.notify(CmdId::PlayerInfoPushCmd, push).await?;
    ctx.send_reply(CmdId::SetShowHeroUniqueIdsCmd, reply, 0, req.up_tag)
        .await
}
