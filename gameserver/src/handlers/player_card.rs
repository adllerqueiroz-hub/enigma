use crate::{
    error::AppError,
    logic::player_card,
    net::{context::ConnectionContext, packet::ClientPacket},
};
use prost::Message;
use sonettobuf::{
    CmdId, GetOtherPlayerCardInfoRequest, SetPlayerCardBadgeRequest,
    SetPlayerCardBaseSettingRequest, SetPlayerCardCritterRequest, SetPlayerCardHeroCoverRequest,
    SetPlayerCardProgressSettingRequest, SetPlayerCardShowAchievementReply,
    SetPlayerCardShowAchievementRequest, SetPlayerCardShowSettingRequest,
    SetPlayerCardThemeRequest,
};

pub async fn on_get_player_card_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let reply = player_card::get_player_card_info(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::GetPlayerCardInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_other_player_card_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetOtherPlayerCardInfoRequest::decode(&req.data[..])?;
    let reply = player_card::get_other_player_card_info(
        ctx.state.db,
        msg.user_id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;

    ctx.send_reply(CmdId::GetOtherPlayerCardInfoCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_player_card_show_setting(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetPlayerCardShowSettingRequest::decode(&req.data[..])?;
    let reply = player_card::set_show_settings(ctx.state.db, player_id, msg.show_settings).await?;
    ctx.send_reply(CmdId::SetPlayerCardShowSettingCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_player_card_progress_setting(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetPlayerCardProgressSettingRequest::decode(&req.data[..])?;
    let reply = player_card::set_progress_setting(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.progress_setting.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::SetPlayerCardProgressSettingCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_player_card_base_setting(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetPlayerCardBaseSettingRequest::decode(&req.data[..])?;
    let reply = player_card::set_base_setting(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.base_setting.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::SetPlayerCardBaseSettingCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_player_card_hero_cover(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetPlayerCardHeroCoverRequest::decode(&req.data[..])?;
    let reply = player_card::set_hero_cover(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.hero_cover.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::SetPlayerCardHeroCoverCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_player_card_theme(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetPlayerCardThemeRequest::decode(&req.data[..])?;
    let reply = player_card::set_theme(
        ctx.state.db,
        ctx.state.tables,
        player_id,
        msg.theme_id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::SetPlayerCardThemeCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_player_card_critter(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetPlayerCardCritterRequest::decode(&req.data[..])?;
    let reply = player_card::set_critter(
        ctx.state.db,
        player_id,
        msg.critter_uid.ok_or(AppError::InvalidRequest)? as i64,
    )
    .await?;
    ctx.send_reply(CmdId::SetPlayerCardCritterCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_player_card_show_achievement(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = SetPlayerCardShowAchievementRequest::decode(&req.data[..])?;
    let db = ctx.state.db;
    let saved = ctx
        .player_mut()?
        .collection
        .show_achievement(db, msg.ids, msg.group_id)
        .await?;
    let reply = SetPlayerCardShowAchievementReply {
        ids: saved.ids,
        group_id: saved.group_id,
    };
    ctx.send_reply(CmdId::SetPlayerCardShowAchievementCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_set_player_card_badge(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SetPlayerCardBadgeRequest::decode(&req.data[..])?;
    let reply =
        player_card::set_badges(ctx.state.db, ctx.state.tables, player_id, msg.badge_ids).await?;
    ctx.send_reply(CmdId::SetPlayerCardBadgeCmd, reply, 0, req.up_tag)
        .await
}
