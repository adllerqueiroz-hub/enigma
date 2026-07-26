use crate::{
    error::AppError,
    logic::hero,
    net::{context::ConnectionContext, packet::ClientPacket},
    util::{push, task_events},
};
use database::db::game::tasks::TaskEvent;
use prost::Message;
use sonettobuf::{
    CancelHero3124TalentTreeRequest, ChoiceHero3123WeaponRequest, ChoiceHero3124TalentTreeRequest,
    CmdId, DestinyLevelUpRequest, DestinyRankUpRequest, DestinyStoneUnlockRequest,
    DestinyStoneUseRequest, GetHeroBirthdayRequest, HeroDefaultEquipRequest, HeroLevelUpRequest,
    HeroRankUpRequest, HeroRedDotReadRequest, HeroTouchRequest, HeroUpgradeSkillRequest,
    ItemUnlockRequest, MarkHeroFavorRequest, ResetHero3124TalentTreeRequest, UnMarkIsNewRequest,
    UnlockVoiceRequest, UseSkinRequest,
};

pub async fn on_unlock_voice(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = UnlockVoiceRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let reply = hero::unlock_voice(
        ctx.state.db,
        player_id,
        hero_id,
        msg.voice_id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;

    push::send_hero_update_push(ctx, player_id, vec![hero_id]).await?;
    ctx.send_reply(CmdId::UnlockVoiceCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_item_unlock(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ItemUnlockRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let (reply, reward) = hero::unlock_item(
        ctx.state.db,
        player_id,
        hero_id,
        msg.item_id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;

    push::send_hero_update_push(ctx, player_id, vec![hero_id]).await?;
    push::send_currency_change_push(ctx, player_id, vec![reward]).await?;
    ctx.send_reply(CmdId::ItemUnlockCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_mark_hero_favor(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = MarkHeroFavorRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let is_favor = msg.is_favor.ok_or(AppError::InvalidRequest)?;
    let reply = hero::mark_favor(ctx.state.db, player_id, hero_id, is_favor).await?;

    push::send_hero_update_push(ctx, player_id, vec![hero_id]).await?;
    ctx.send_reply(CmdId::MarkHeroFavorCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_unmark_is_new(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = UnMarkIsNewRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let reply = hero::unmark_new(ctx.state.db, player_id, hero_id).await?;

    push::send_hero_update_push(ctx, player_id, vec![hero_id]).await?;
    ctx.send_reply(CmdId::UnMarkIsNewCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_use_skin(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = UseSkinRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let skin_id = msg.skin_id.ok_or(AppError::InvalidRequest)?;
    let reply = hero::use_skin(ctx.state.db, player_id, hero_id, skin_id).await?;

    push::send_hero_update_push(ctx, player_id, vec![hero_id]).await?;
    ctx.send_reply(CmdId::UseSkinCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_hero_red_dot_read(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = HeroRedDotReadRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let red_dot = msg.red_dot_type.ok_or(AppError::InvalidRequest)?;
    let reply = hero::read_red_dot(ctx.state.db, player_id, hero_id, red_dot).await?;

    push::send_hero_update_push(ctx, player_id, vec![hero_id]).await?;
    ctx.send_reply(CmdId::HeroRedDotReadCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_hero_touch(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = HeroTouchRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let reply = hero::touch(ctx.state.db, player_id, hero_id).await?;

    push::send_hero_update_push(ctx, player_id, vec![hero_id]).await?;
    task_events::notify(
        ctx,
        player_id,
        TaskEvent::DoneCount {
            name: "HeroTouch",
            count: 1,
        },
    )
    .await?;
    ctx.send_reply(CmdId::HeroTouchCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_hero_default_equip(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = HeroDefaultEquipRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let equip_uid = msg.default_equip_uid.ok_or(AppError::InvalidRequest)?;
    let (reply, hero_info) =
        hero::default_equip(ctx.state.db, player_id, hero_id, equip_uid).await?;

    push::send_hero_updates(ctx, player_id, vec![hero_info]).await?;
    ctx.send_reply(CmdId::HeroDefaultEquipCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_hero_birthday(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let msg = GetHeroBirthdayRequest::decode(&req.data[..])?;
    let reply = hero::birthday(msg.hero_id.ok_or(AppError::InvalidRequest)?);

    ctx.send_reply(CmdId::GetHeroBirthdayCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_choice_hero_3123_weapon(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ChoiceHero3123WeaponRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let main_id = msg.main_id.ok_or(AppError::InvalidRequest)?;
    let sub_id = msg.sub_id.ok_or(AppError::InvalidRequest)?;
    let (reply, hero_info) =
        hero::choice_hero_3123_weapon(ctx.state.db, player_id, hero_id, main_id, sub_id).await?;

    push::send_hero_updates(ctx, player_id, vec![hero_info]).await?;
    ctx.send_reply(CmdId::ChoiceHero3123WeaponCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_choice_hero_3124_talent_tree(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ChoiceHero3124TalentTreeRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let sub_id = msg.sub_id.ok_or(AppError::InvalidRequest)?;
    let level = msg.level.ok_or(AppError::InvalidRequest)?;
    let (reply, hero_info) =
        hero::choice_hero_3124_talent_tree(ctx.state.db, player_id, hero_id, sub_id, level).await?;

    push::send_hero_updates(ctx, player_id, vec![hero_info]).await?;
    ctx.send_reply(CmdId::ChoiceHero3124TalentTreeCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_cancel_hero_3124_talent_tree(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = CancelHero3124TalentTreeRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let sub_id = msg.sub_id.ok_or(AppError::InvalidRequest)?;
    let level = msg.level.ok_or(AppError::InvalidRequest)?;
    let (reply, hero_info) =
        hero::cancel_hero_3124_talent_tree(ctx.state.db, player_id, hero_id, sub_id, level).await?;

    push::send_hero_updates(ctx, player_id, vec![hero_info]).await?;
    ctx.send_reply(CmdId::CancelHero3124TalentTreeCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_reset_hero_3124_talent_tree(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = ResetHero3124TalentTreeRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let (reply, hero_info) =
        hero::reset_hero_3124_talent_tree(ctx.state.db, player_id, hero_id).await?;

    push::send_hero_updates(ctx, player_id, vec![hero_info]).await?;
    ctx.send_reply(CmdId::ResetHero3124TalentTreeCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_destiny_stone_use(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = DestinyStoneUseRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let stone_id = msg.stone_id.ok_or(AppError::InvalidRequest)?;
    let (reply, hero_info) =
        hero::destiny_stone(ctx.state.db, player_id, hero_id, stone_id).await?;

    push::send_hero_updates(ctx, player_id, vec![hero_info]).await?;
    ctx.send_reply(CmdId::DestinyStoneUseCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_destiny_rank_up(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = DestinyRankUpRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let (reply, hero_info, consumed) =
        hero::destiny_rank_up(ctx.state.db, player_id, hero_id).await?;

    push::send_cost_pushes(
        ctx,
        player_id,
        consumed.item_ids,
        consumed.currency_ids,
        consumed.material_changes,
    )
    .await?;
    push::send_hero_updates(ctx, player_id, vec![hero_info]).await?;
    ctx.send_reply(CmdId::DestinyRankUpCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_destiny_level_up(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = DestinyLevelUpRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let level = msg.level.ok_or(AppError::InvalidRequest)?;
    let (reply, hero_info, consumed) =
        hero::destiny_level_up(ctx.state.db, player_id, hero_id, level).await?;

    push::send_cost_pushes(
        ctx,
        player_id,
        consumed.item_ids,
        consumed.currency_ids,
        consumed.material_changes,
    )
    .await?;
    push::send_hero_updates(ctx, player_id, vec![hero_info]).await?;
    ctx.send_reply(CmdId::DestinyLevelUpCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_destiny_stone_unlock(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = DestinyStoneUnlockRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let stone_id = msg.stone_id.ok_or(AppError::InvalidRequest)?;
    let (reply, hero_info, consumed) =
        hero::destiny_stone_unlock(ctx.state.db, player_id, hero_id, stone_id).await?;

    push::send_cost_pushes(
        ctx,
        player_id,
        consumed.item_ids,
        consumed.currency_ids,
        consumed.material_changes,
    )
    .await?;
    push::send_hero_updates(ctx, player_id, vec![hero_info]).await?;
    ctx.send_reply(CmdId::DestinyStoneUnlockCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_hero_level_up(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = HeroLevelUpRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let new_level = msg.expect_level.ok_or(AppError::InvalidRequest)?;
    let (reply, hero_info) = hero::level_up(ctx.state.db, player_id, hero_id, new_level).await?;

    push::send_hero_updates(ctx, player_id, vec![hero_info]).await?;
    task_events::notify(
        ctx,
        player_id,
        TaskEvent::DoneCount {
            name: "HeroLevelUp",
            count: 1,
        },
    )
    .await?;
    ctx.send_reply(CmdId::HeroLevelUpCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_hero_rank_up(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = HeroRankUpRequest::decode(&req.data[..])?;
    let hero_id = msg.hero_id.ok_or(AppError::InvalidRequest)?;
    let (reply, hero_info) = hero::rank_up(ctx.state.db, player_id, hero_id).await?;

    push::send_hero_updates(ctx, player_id, vec![hero_info]).await?;
    ctx.send_reply(CmdId::HeroRankUpCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_hero_upgrade_skill(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = HeroUpgradeSkillRequest::decode(&req.data[..])?;
    let (reply, hero_info, consumed_item_id) = hero::upgrade_skill(
        ctx.state.db,
        player_id,
        msg.hero_id,
        msg.r#type,
        msg.consume.unwrap_or(1),
    )
    .await?;

    push::send_item_change_push(
        ctx,
        player_id,
        vec![consumed_item_id],
        Vec::new(),
        Vec::new(),
    )
    .await?;
    push::send_hero_updates(ctx, player_id, vec![hero_info]).await?;
    ctx.send_reply(CmdId::HeroUpgradeSkillCmd, reply, 0, req.up_tag)
        .await
}
