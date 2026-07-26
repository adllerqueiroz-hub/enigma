use crate::{
    error::AppError,
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
    let reply = ctx.player()?.profile.get_player_info(ctx.state.db).await?;

    ctx.send_reply(CmdId::GetPlayerInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_other_player_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetOtherPlayerInfoRequest::decode(&req.data[..])?;
    let reply = ctx
        .player()?
        .profile
        .get_other_player_info(ctx.state.db, msg.user_id.ok_or(AppError::InvalidRequest)?)
        .await?;

    ctx.send_reply(CmdId::GetOtherPlayerInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_open_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let profile = ctx.player()?.profile;
    let msg = GetOpenInfoRequest::decode(&req.data[..])?;
    let reply = profile.get_open_info(ctx.state.db, msg.id).await?;

    ctx.send_reply(CmdId::GetOpenInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_hero_info_list(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = ctx.player()?.profile.hero_info_list(ctx.state.db).await?;

    ctx.send_reply(CmdId::HeroInfoListCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_portrait(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let profile = ctx.player()?.profile;
    let msg = SetPortraitRequest::decode(&req.data[..])?;
    let portrait = msg.portrait.ok_or(AppError::InvalidRequest)?;
    let reply = profile.set_portrait(ctx.state.db, portrait).await?;

    ctx.send_reply(CmdId::SetPortraitCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_signature(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let profile = ctx.player()?.profile;
    let msg = SetSignatureRequest::decode(&req.data[..])?;
    let reply = profile
        .set_signature(
            ctx.state.db,
            ctx.state.tables,
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
    let profile = ctx.player()?.profile;
    let msg = SetBirthdayRequest::decode(&req.data[..])?;
    let reply = profile
        .set_birthday(
            ctx.state.db,
            ctx.state.tables,
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
    let profile = ctx.player()?.profile;
    let msg = SetCharacterAgeRequest::decode(&req.data[..])?;
    let reply = profile
        .set_character_age(ctx.state.db, ctx.state.tables, msg.character_age)
        .await?;
    ctx.send_reply(CmdId::SetCharacterAgeCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_player_bg(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let profile = ctx.player()?.profile;
    let msg = SetPlayerBgRequest::decode(&req.data[..])?;
    let reply = profile
        .set_player_bg(
            ctx.state.db,
            ctx.state.tables,
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
    let profile = ctx.player()?.profile;
    let msg = SetShowHeroUniqueIdsRequest::decode(&req.data[..])?;
    let (reply, push) = profile
        .set_show_hero_unique_ids(ctx.state.db, msg.show_hero_unique_ids)
        .await?;

    ctx.notify(CmdId::PlayerInfoPushCmd, push).await?;
    ctx.send_reply(CmdId::SetShowHeroUniqueIdsCmd, reply, 0, req.up_tag)
        .await
}
