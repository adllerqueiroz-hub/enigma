use crate::{
    error::AppError,
    logic::{dungeon, tower, tower_compose},
    net::context::ConnectionContext,
    net::packet::ClientPacket,
    player::battle::ActiveBattle,
    util::{push, task_events},
};
use config::configs;
use database::db::game::{battle as battle_db, dungeons, tasks::TaskEvent};
use prost::Message;
use sonettobuf::{
    AutoRoundReply, AutoRoundRequest, BeginRoundRequest, CmdId, CoverDungeonRecordReply,
    CoverDungeonRecordRequest, DungeonInfosPush, EndDungeonReply, EndDungeonRequest, EndFightReply,
    EndFightRequest, EntityInfoReply, EntityInfoRequest, GetFightCardDeckDetailInfoReply,
    GetFightCardDeckDetailInfoRequest, GetFightCardDeckInfoReply, GetFightCardDeckInfoRequest,
    GetFightOperReply, GetFightOperRequest, GetFightRecordGroupReply, GetFightRecordGroupRequest,
    GetPuzzleProgressRequest, InstructionDungeonFinalRewardRequest, InstructionDungeonInfoRequest,
    InstructionDungeonOpenRequest, InstructionDungeonRewardRequest, PuzzleFinishRequest,
    ReconnectFightRequest, RefreshAssistRequest, ResetRoundReply, ResetRoundRequest,
    SavePuzzleProgressRequest, StartDungeonReply, StartDungeonRequest, UpdateOpenPush,
    UseClothSkillRequest,
};

pub async fn on_refresh_assist(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let request = RefreshAssistRequest::decode(&req.data[..])?;
    let reply = dungeon::refresh_assist(ctx.state.db, player_id, request).await?;
    ctx.send_reply(CmdId::RefreshAssistCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_entity_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = EntityInfoRequest::decode(&req.data[..])?;
    let entity_info = ctx
        .player()?
        .battle
        .active
        .as_ref()
        .ok_or(AppError::InvalidRequest)?
        .runtime
        .entity_info(request.uid.ok_or(AppError::InvalidRequest)?)
        .cloned()
        .ok_or(AppError::InvalidRequest)?;
    ctx.send_reply(
        CmdId::EntityInfoCmd,
        EntityInfoReply {
            entity_info: Some(entity_info),
        },
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_get_fight_card_deck_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = GetFightCardDeckInfoRequest::decode(&req.data[..])?;
    let deck_infos = active_card_deck(ctx, request.r#type.unwrap_or_default())?;
    ctx.send_reply(
        CmdId::GetFightCardDeckInfoCmd,
        GetFightCardDeckInfoReply {
            deck_infos,
            device_infos: Vec::new(),
        },
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_get_fight_card_deck_detail_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = GetFightCardDeckDetailInfoRequest::decode(&req.data[..])?;
    let deck_infos = active_card_deck(ctx, request.r#type.unwrap_or_default())?;
    ctx.send_reply(
        CmdId::GetFightCardDeckDetailInfoCmd,
        GetFightCardDeckDetailInfoReply {
            deck_infos,
            device_infos: Vec::new(),
        },
        0,
        req.up_tag,
    )
    .await
}

fn active_card_deck(
    ctx: &ConnectionContext,
    team_type: i32,
) -> Result<Vec<sonettobuf::CardInfo>, AppError> {
    ctx.player()?
        .battle
        .active
        .as_ref()
        .ok_or(AppError::InvalidRequest)?
        .runtime
        .card_deck(team_type)
        .map(<[_]>::to_vec)
        .ok_or(AppError::InvalidRequest)
}

pub async fn on_get_puzzle_progress(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = GetPuzzleProgressRequest::decode(&req.data[..])?;
    let reply = dungeon::get_puzzle_progress(
        ctx.state.db,
        player_id,
        msg.element_id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::GetPuzzleProgressCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_save_puzzle_progress(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = SavePuzzleProgressRequest::decode(&req.data[..])?;
    let reply = dungeon::save_puzzle_progress(
        ctx.state.db,
        player_id,
        msg.element_id.ok_or(AppError::InvalidRequest)?,
        msg.progress.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::SavePuzzleProgressCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_puzzle_finish(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = PuzzleFinishRequest::decode(&req.data[..])?;
    let reply = dungeon::finish_puzzle(
        ctx.state.db,
        player_id,
        msg.element_id.ok_or(AppError::InvalidRequest)?,
    )
    .await?;
    ctx.send_reply(CmdId::PuzzleFinishCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_reset_round(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    ResetRoundRequest::decode(&req.data[..])?;
    if ctx.player()?.battle.active.is_none() {
        return Err(AppError::InvalidRequest);
    }
    ctx.send_reply(
        CmdId::ResetRoundCmd,
        ResetRoundReply::default(),
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_use_cloth_skill(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = UseClothSkillRequest::decode(&req.data[..])?;
    let (reply, redeal) = {
        let active = ctx
            .player_mut()?
            .battle
            .active
            .as_mut()
            .ok_or(AppError::InvalidRequest)?;
        active.use_cloth_skill(request)?
    };
    if let Some(redeal) = redeal {
        ctx.notify(CmdId::RedealCardInfoPushCmd, redeal).await?;
    }
    ctx.send_reply(CmdId::UseClothSkillCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_get_dungeon(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let (reply, dungeons) = dungeon::dungeon_info(ctx.state.db, player_id).await?;

    ctx.send_reply(CmdId::GetDungeonCmd, reply, 0, req.up_tag)
        .await?;

    for chunk in dungeons.chunks(100) {
        ctx.notify(
            CmdId::DungeonInfosPushCmd,
            DungeonInfosPush {
                dungeon_infos: chunk.iter().cloned().map(Into::into).collect(),
            },
        )
        .await?;
    }
    Ok(())
}

pub async fn on_start_dungeon(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    ctx.player()?
        .battle
        .ensure_can_start(ctx.state.db, player_id)
        .await?;
    let request = StartDungeonRequest::decode(&req.data[..])?;

    let chapter_id = request
        .chapter_id
        .unwrap_or_else(|| episode_cfg_chapter_id(request.episode_id.unwrap_or(0)));
    let episode_id = request.episode_id.unwrap_or(0);
    let multiplier = request.multiplication.unwrap_or(1).max(1);

    let game_data = configs::get();
    let episode_cfg = game_data
        .episode
        .iter()
        .find(|e| e.id == episode_id)
        .ok_or(AppError::InvalidRequest)?;
    if !dungeons::can_start_episode(ctx.state.db, player_id, chapter_id, episode_id).await? {
        return Err(AppError::InvalidRequest);
    }

    if episode_cfg.battle_id == 0 {
        let settlement = dungeon::settle_battleless(
            ctx.state.db,
            player_id,
            chapter_id,
            episode_id,
            dungeon::DungeonCompletion {
                star: 1,
                total_round: 0,
                multiplier,
                fight_group: None,
            },
            &Default::default(),
        )
        .await?;
        push::send_cost_pushes(
            ctx,
            player_id,
            settlement.cost.item_ids,
            settlement.cost.currency_ids,
            settlement.cost.material_changes,
        )
        .await?;
        ctx.player_mut()?.battle.pending_record = None;
        send_dungeon_settlement(ctx, player_id, settlement.dungeon).await?;

        return ctx
            .send_reply(
                CmdId::StartDungeonCmd,
                StartDungeonReply {
                    fight: None,
                    round: None,
                },
                0,
                req.up_tag,
            )
            .await;
    }

    let mut active = ActiveBattle::prepare(
        ctx.state.db,
        player_id,
        episode_id,
        episode_cfg.battle_id,
        request,
    )
    .await?;
    let cost = active
        .activate(
            ctx.state.db,
            player_id,
            &dungeon::episode_cost(episode_cfg, multiplier),
        )
        .await?;
    let reply = active.start_reply();
    let cards = active.card_info_push();
    let battle = &mut ctx.player_mut()?.battle;
    battle.pending_record = None;
    battle.active = Some(active);

    push::send_cost_pushes(
        ctx,
        player_id,
        cost.item_ids,
        cost.currency_ids,
        cost.material_changes,
    )
    .await?;
    ctx.send_reply(CmdId::StartDungeonCmd, reply, 0, req.up_tag)
        .await?;
    ctx.notify(CmdId::CardInfoPushCmd, cards).await
}

pub async fn on_reconnect_fight(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    ReconnectFightRequest::decode(&req.data[..])?;
    let mut refund = None;

    if ctx.player()?.battle.active.is_none()
        && let Some(record) = battle_db::load_active_fight(ctx.state.db, player_id).await?
    {
        let fight_id = record.id;
        let episode_id = record.episode_id;
        let multiplication = record.multiplication;
        let entry_cost = record.entry_cost.clone();
        match ActiveBattle::restore(ctx.state.db, player_id, record).await {
            Ok(active) => ctx.player_mut()?.battle.active = Some(active),
            Err(AppError::InvalidBattleCheckpoint(error)) => {
                tracing::warn!(player_id, fight_id, %error, "refunding invalid fight checkpoint");
                let entry_cost = if entry_cost.is_empty() {
                    let episode = configs::get()
                        .episode
                        .get(episode_id)
                        .ok_or(AppError::InvalidRequest)?;
                    dungeon::failure_refund(episode, multiplication)
                } else {
                    serde_json::from_str(&entry_cost)
                        .map_err(|error| AppError::InvalidBattleCheckpoint(error.to_string()))?
                };
                refund = Some(
                    dungeon::settle_checkpoint_refund(
                        ctx.state.db,
                        player_id,
                        fight_id,
                        entry_cost,
                    )
                    .await?,
                );
            }
            Err(error) => return Err(error),
        }
    }
    if let Some(refund) = refund {
        send_refund(ctx, player_id, refund).await?;
    }
    let reply = ctx
        .player()?
        .battle
        .active
        .as_ref()
        .map(ActiveBattle::reconnect_reply)
        .unwrap_or_default();
    ctx.send_reply(CmdId::ReconnectFightCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_instruction_dungeon_info(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    InstructionDungeonInfoRequest::decode(&req.data[..])?;
    let reply = dungeon::instruction_dungeon_info(ctx.state.db, player_id).await?;

    ctx.send_reply(
        CmdId::DungeonInstructionDungeonInfoCmd,
        reply,
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_instruction_dungeon_open(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = InstructionDungeonOpenRequest::decode(&req.data[..])?;
    let reply = dungeon::instruction_dungeon_open(ctx.state.db, player_id, msg.open_id).await?;

    push::send_instruction_dungeon_info(ctx, player_id).await?;
    ctx.send_reply(CmdId::InstructionDungeonOpenCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_instruction_dungeon_reward(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = InstructionDungeonRewardRequest::decode(&req.data[..])?;
    let claim = dungeon::instruction_dungeon_reward(
        ctx.state.db,
        player_id,
        msg.topic_id.unwrap_or_default(),
    )
    .await?;

    push::send_applied_reward_pushes(ctx, player_id, claim.rewards, claim.material_changes, None)
        .await?;
    push::send_instruction_dungeon_info(ctx, player_id).await?;
    ctx.send_reply(
        CmdId::InstructionDungeonRewardCmd,
        claim.reply,
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_instruction_dungeon_final_reward(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    InstructionDungeonFinalRewardRequest::decode(&req.data[..])?;
    let claim = dungeon::instruction_dungeon_final_reward(ctx.state.db, player_id).await?;

    push::send_applied_reward_pushes(ctx, player_id, claim.rewards, claim.material_changes, None)
        .await?;
    push::send_instruction_dungeon_info(ctx, player_id).await?;
    ctx.send_reply(
        CmdId::InstructionDungeonFinalRewardCmd,
        claim.reply,
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_begin_round(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = BeginRoundRequest::decode(&req.data[..])?;
    let reply = {
        let player = ctx.player_mut()?;
        let active = player
            .battle
            .active
            .as_mut()
            .ok_or(AppError::InvalidRequest)?;
        active.begin_round(request)?
    };

    ctx.send_reply(CmdId::BeginRoundCmd, reply, 0, req.up_tag)
        .await
}

pub async fn on_auto_round(ctx: &mut ConnectionContext, req: ClientPacket) -> Result<(), AppError> {
    let devices_opers = ctx
        .player()?
        .battle
        .active
        .as_ref()
        .map(|battle| battle.runtime.conduit_operations())
        .unwrap_or_default();
    let msg = AutoRoundRequest::decode(&req.data[..])?;

    ctx.send_reply(
        CmdId::AutoRoundCmd,
        AutoRoundReply {
            opers: msg.opers,
            to_id: msg.to_id,
            cloth_skill: None,
            devices_opers,
        },
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_fight_end_fight(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = EndFightRequest::decode(&req.data[..])?;
    let active = ctx.player()?.battle.active.clone();
    let end = active.as_ref().map(|active| {
        if msg.is_abort.unwrap_or(false) {
            dungeon::abort_end_fight(active)
        } else {
            dungeon::completed_end_fight(active)
        }
    });

    let mut compose_push = None;
    let mut dungeon_settlement = None;
    let mut refund_settlement = None;
    let mut completed_dungeon = None;
    if let Some(active) = active.as_ref() {
        let is_abort = msg.is_abort.unwrap_or(false);
        let compose_handled = tower_compose::matches_battle(active);
        let won = active.runtime.outcome() == battle::engine::runtime::BattleOutcome::Victory;

        if !is_abort && won && (compose_handled || active.tower_type.is_none()) {
            let star = dungeon::battle_star(&active.runtime, active.battle_id);
            let round = active.runtime.current_round();
            let record =
                dungeon::prepare_dungeon_record(ctx.state.db, player_id, active, round).await?;
            let mut settlement = dungeon::settle_active(
                ctx.state.db,
                player_id,
                active,
                dungeon::DungeonCompletion {
                    star,
                    total_round: round,
                    multiplier: active.multiplication.unwrap_or(1).max(1),
                    fight_group: active.fight_group.as_ref(),
                },
                &record,
            )
            .await?;
            compose_push = settlement.compose_push.take();
            ctx.player_mut()?.battle.pending_record = record.pending;
            dungeon_settlement = Some(settlement);
            completed_dungeon = Some((active.chapter_id, active.episode_id));
        } else if is_abort || !won {
            let mut settlement =
                dungeon::settle_refund(ctx.state.db, player_id, active, !is_abort).await?;
            compose_push = settlement.compose_push.take();
            refund_settlement = Some(settlement);
        } else if let Some(fight_id) = active.fight_id {
            battle_db::finish_fight_instance(ctx.state.db, player_id, fight_id).await?;
        }

        ctx.player_mut()?.battle.active = None;
    }
    if let Some(settle) = compose_push {
        ctx.notify(CmdId::TowerComposeFightSettlePushCmd, settle)
            .await?;
    }
    if let Some(settlement) = dungeon_settlement {
        send_dungeon_settlement(ctx, player_id, settlement).await?;
    }
    if let Some(settlement) = refund_settlement {
        send_refund(ctx, player_id, settlement).await?;
    }
    if let Some(end) = end {
        push::send_end_fight_push(ctx, end).await?;
    }
    if let Some((chapter_id, episode_id)) = completed_dungeon {
        notify_dungeon_completion_tasks(ctx, player_id, chapter_id, episode_id).await?;
    }
    ctx.send_reply(CmdId::FightEndFightCmd, EndFightReply {}, 0, req.up_tag)
        .await
}

pub async fn on_get_fight_record_group(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let request = GetFightRecordGroupRequest::decode(&req.data[..])?;
    let fight_group = dungeons::load_dungeon_record(
        ctx.state.db,
        player_id,
        request.episode_id.unwrap_or_default(),
    )
    .await?;

    ctx.send_reply(
        CmdId::GetFightRecordGroupCmd,
        GetFightRecordGroupReply { fight_group },
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_get_fight_oper(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    GetFightOperRequest::decode(&req.data[..])?;
    let episode_id = ctx
        .player()?
        .battle
        .active
        .as_ref()
        .filter(|active| active.is_replay.unwrap_or(false))
        .map(|active| active.episode_id)
        .ok_or(AppError::InvalidRequest)?;
    let oper_records =
        dungeons::load_dungeon_record_operations(ctx.state.db, player_id, episode_id).await?;

    ctx.send_reply(
        CmdId::GetFightOperCmd,
        GetFightOperReply { oper_records },
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_dungeon_end_dungeon(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let msg = EndDungeonRequest::decode(&req.data[..])?;
    let active = ctx.player()?.battle.active.clone();

    if let Some(active) = active.as_ref() {
        if msg.is_abort.unwrap_or(false) {
            let tower_push = tower::abort_finish_push(ctx.state.db, player_id, active).await?;
            let (dungeon_update, end_dungeon) =
                dungeon::abort_dungeon_updates(ctx.state.db, player_id, active).await?;
            let refund = dungeon::settle_refund(ctx.state.db, player_id, active, false).await?;
            ctx.player_mut()?.battle.active = None;
            send_refund(ctx, player_id, refund).await?;
            if let Some(tower_push) = tower_push {
                ctx.notify(CmdId::TowerBattleFinishPushCmd, tower_push)
                    .await?;
            }
            push::send_instruction_dungeon_info(ctx, player_id).await?;
            ctx.notify(CmdId::DungeonUpdatePushCmd, dungeon_update)
                .await?;
            ctx.notify(CmdId::DungeonEndDungeonPushCmd, end_dungeon)
                .await?;
        } else if active.runtime.outcome() == battle::engine::runtime::BattleOutcome::Victory {
            let star = dungeon::battle_star(&active.runtime, active.battle_id);
            let round = active.runtime.current_round();
            let record =
                dungeon::prepare_dungeon_record(ctx.state.db, player_id, active, round).await?;
            let mut settlement = dungeon::settle_active(
                ctx.state.db,
                player_id,
                active,
                dungeon::DungeonCompletion {
                    star,
                    total_round: round,
                    multiplier: active.multiplication.unwrap_or(1).max(1),
                    fight_group: active.fight_group.as_ref(),
                },
                &record,
            )
            .await?;
            ctx.player_mut()?.battle.pending_record = record.pending;
            ctx.player_mut()?.battle.active = None;
            if let Some(compose) = settlement.compose_push.take() {
                ctx.notify(CmdId::TowerComposeFightSettlePushCmd, compose)
                    .await?;
            }
            send_dungeon_settlement(ctx, player_id, settlement).await?;
            notify_dungeon_completion_tasks(ctx, player_id, active.chapter_id, active.episode_id)
                .await?;
        } else {
            let mut refund = dungeon::settle_refund(ctx.state.db, player_id, active, true).await?;
            ctx.player_mut()?.battle.active = None;
            if let Some(compose) = refund.compose_push.take() {
                ctx.notify(CmdId::TowerComposeFightSettlePushCmd, compose)
                    .await?;
            }
            send_refund(ctx, player_id, refund).await?;
            push::send_end_fight_push(ctx, dungeon::completed_end_fight(active)).await?;
        }
    }

    ctx.send_reply(
        CmdId::DungeonEndDungeonCmd,
        EndDungeonReply {},
        0,
        req.up_tag,
    )
    .await
}

pub async fn on_cover_dungeon_record(
    ctx: &mut ConnectionContext,
    req: ClientPacket,
) -> Result<(), AppError> {
    let player_id = ctx.player()?.id;
    let request = CoverDungeonRecordRequest::decode(&req.data[..])?;
    let pending = ctx.player_mut()?.battle.pending_record.take();
    let is_cover = dungeon::cover_dungeon_record(
        ctx.state.db,
        player_id,
        pending,
        request.is_cover.unwrap_or(false),
    )
    .await?;

    ctx.send_reply(
        CmdId::CoverDungeonRecordCmd,
        CoverDungeonRecordReply {
            is_cover: Some(is_cover),
        },
        0,
        req.up_tag,
    )
    .await
}

fn episode_cfg_chapter_id(episode_id: i32) -> i32 {
    configs::get()
        .episode
        .get(episode_id)
        .map(|episode| episode.chapter_id)
        .unwrap_or_default()
}

async fn send_dungeon_settlement(
    ctx: &mut ConnectionContext,
    player_id: i64,
    settlement: dungeon::DungeonSettlement,
) -> Result<(), AppError> {
    push::send_hero_update_push(ctx, player_id, settlement.hero_ids).await?;
    push::send_dungeon_completion_reward_pushes(ctx, player_id, settlement.rewards).await?;
    push::send_dungeon_map_progression(ctx, player_id).await?;
    push::send_instruction_dungeon_progression(ctx, player_id).await?;
    ctx.notify(CmdId::DungeonUpdatePushCmd, settlement.dungeon_update)
        .await?;
    if !settlement.open_infos.is_empty() {
        ctx.notify(
            CmdId::UpdateOpenPushCmd,
            UpdateOpenPush {
                open_infos: settlement.open_infos,
            },
        )
        .await?;
    }
    ctx.notify(CmdId::DungeonEndDungeonPushCmd, settlement.end_dungeon)
        .await
}

async fn notify_dungeon_completion_tasks(
    ctx: &mut ConnectionContext,
    player_id: i64,
    chapter_id: i32,
    episode_id: i32,
) -> Result<(), AppError> {
    for chapter_type in dungeon::dungeon_pass_types(chapter_id) {
        task_events::notify(
            ctx,
            player_id,
            TaskEvent::DungeonPass {
                chapter_type,
                count: 1,
            },
        )
        .await?;
    }
    task_events::notify(
        ctx,
        player_id,
        TaskEvent::DoneCount {
            name: "DungeonPass",
            count: 1,
        },
    )
    .await?;
    task_events::notify(ctx, player_id, TaskEvent::EpisodeFinish { episode_id }).await
}

async fn send_refund(
    ctx: &mut ConnectionContext,
    player_id: i64,
    settlement: dungeon::RefundSettlement,
) -> Result<(), AppError> {
    push::send_applied_reward_pushes(
        ctx,
        player_id,
        settlement.rewards,
        settlement.material_changes,
        None,
    )
    .await
}
