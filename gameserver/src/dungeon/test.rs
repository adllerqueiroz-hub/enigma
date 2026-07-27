use super::*;

#[tokio::test]
async fn first_clear_uses_the_configured_first_battle() {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/excel2json");
    let _ = config::init(data_dir.to_str().unwrap());
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    let episode = configs::get().episode.get(10103).unwrap();

    assert_eq!(episode_battle_id(&pool, 1, episode).await.unwrap(), 11021);

    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at) VALUES (1, 'first', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO user_dungeons
             (user_id, chapter_id, episode_id, star, challenge_count, has_record,
              left_return_all_num, today_pass_num, today_total_num, created_at, updated_at)
             VALUES (1, 101, 10103, 1, 0, 0, 1, 0, 0, 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();

    assert_eq!(episode_battle_id(&pool, 1, episode).await.unwrap(), 1102);
    let episode_without_first_battle = configs::get().episode.get(10001).unwrap();
    assert_eq!(
        episode_battle_id(&pool, 1, episode_without_first_battle)
            .await
            .unwrap(),
        1001
    );
}

#[test]
fn saved_start_matches_with_or_without_the_restart_marker() {
    let start = sonettobuf::StartDungeonRequest {
        chapter_id: Some(301),
        episode_id: Some(10002),
        fight_group: Some(Default::default()),
        multiplication: Some(1),
        ..Default::default()
    };
    let active = ActiveBattle {
        start_request: Some(start.clone()),
        ..Default::default()
    };
    let mut restart = start;

    assert!(matches_saved_dungeon_start(&active, &restart));
    restart.is_restart = Some(true);
    assert!(matches_saved_dungeon_start(&active, &restart));
    restart.episode_id = Some(10003);
    assert!(!matches_saved_dungeon_start(&active, &restart));

    restart.episode_id = Some(10002);
    let tower = ActiveBattle {
        start_request: active.start_request.clone(),
        tower_context: Some(::battle::tower::BattleContext {
            tower_type: 1,
            tower_id: 2,
            layer_id: 3,
            difficulty: 4,
            talent_plan_id: 5,
        }),
        ..Default::default()
    };
    assert!(!matches_saved_dungeon_start(&tower, &restart));
}

#[test]
fn abort_push_carries_the_abort_result_and_required_fight_group() {
    let push = abort_end_fight(&ActiveBattle {
        fight_id: Some(42),
        fight_group: Some(Default::default()),
        is_replay: Some(false),
        ..Default::default()
    });

    let record = push.record.unwrap();
    assert_eq!(record.fight_result, Some(-1));
    assert_eq!(record.fight_name.as_deref(), Some(""));
    assert!(record.fight_time.is_some());
    assert!(push.fight_group_a.is_some());
    assert_eq!(push.is_record, Some(false));
}

#[tokio::test]
async fn dungeon_abort_sends_terminal_fight_push_before_reply() {
    use crate::{
        handlers::dungeon::on_dungeon_end_dungeon,
        net::{
            app::AppState, context::ConnectionContext, outbound::CommandPacket,
            packet::ClientPacket,
        },
        player::{Player, PlayerState},
    };
    use prost::Message;
    use sonettobuf::CmdId;
    use tokio::sync::mpsc;

    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/excel2json");
    let _ = config::init(data_dir.to_str().unwrap());
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at)
         VALUES (29, 'abort-push', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    database::db::starter_data::load_all_starter_data(&pool, 29)
        .await
        .unwrap();

    let episode = configs::get().episode.get(10001).unwrap();
    let fight_id = battle_db::create_fight_instance(
        &pool,
        battle_db::NewFightInstance {
            user_id: 29,
            episode_id: episode.id,
            battle_id: episode.battle_id,
            multiplication: 1,
            entry_cost: "{}",
            checkpoint: "{}",
            created_at: 0,
        },
    )
    .await
    .unwrap();
    let active = ActiveBattle {
        fight_id: Some(fight_id),
        chapter_id: episode.chapter_id,
        episode_id: episode.id,
        battle_id: episode.battle_id,
        fight_group: Some(Default::default()),
        ..Default::default()
    };

    let state = Box::leak(Box::new(AppState::new(pool, configs::get())));
    let (outbound, mut packets) = mpsc::channel(32);
    let mut ctx = ConnectionContext::new(outbound, state);
    ctx.player = Some(Player::new(29, PlayerState::new(29, 0)));
    ctx.player_mut().unwrap().battle.restore_active(active);

    let mut data = Vec::new();
    sonettobuf::EndDungeonRequest {
        is_abort: Some(true),
        ..Default::default()
    }
    .encode(&mut data)
    .unwrap();
    on_dungeon_end_dungeon(
        &mut ctx,
        ClientPacket {
            sequence: 0,
            cmd_id: CmdId::DungeonEndDungeonCmd as i16,
            up_tag: 7,
            data,
        },
    )
    .await
    .unwrap();

    let mut end_dungeon = None;
    let mut end_fight = None;
    let mut reply = None;
    let mut position = 0;
    while let Ok(packet) = packets.try_recv() {
        match packet {
            CommandPacket::Push {
                cmd_id: CmdId::DungeonEndDungeonPushCmd,
                ..
            } => {
                end_dungeon = Some(position);
            }
            CommandPacket::Push {
                cmd_id: CmdId::FightEndFightPushCmd,
                body,
                ..
            } => {
                let push = sonettobuf::EndFightPush::decode(&*body).unwrap();
                assert_eq!(
                    push.record.unwrap().fight_result,
                    Some(FightResult::Abort as i32)
                );
                end_fight = Some(position);
            }
            CommandPacket::Reply {
                cmd_id: CmdId::DungeonEndDungeonCmd,
                ..
            } => {
                reply = Some(position);
            }
            _ => {}
        }
        position += 1;
    }

    let end_dungeon = end_dungeon.unwrap();
    let end_fight = end_fight.unwrap();
    let reply = reply.unwrap();
    assert!(end_dungeon < end_fight);
    assert!(end_fight < reply);
    assert!(!ctx.player().unwrap().battle.has_active());
}

#[tokio::test]
async fn abort_dungeon_keeps_saved_progress_but_reports_no_new_star() {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/excel2json");
    let _ = config::init(data_dir.to_str().unwrap());
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at) VALUES (18, 'abort', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    let active = ActiveBattle {
        chapter_id: 9000,
        episode_id: 90002501,
        runtime: battle::engine::runtime::BattleRuntime::new(sonettobuf::Fight {
            cur_round: Some(15),
            ..Default::default()
        }),
        ..Default::default()
    };

    let (update, end) = abort_dungeon_updates(&pool, 18, &active).await.unwrap();
    assert_eq!(update.dungeon_info.unwrap().star, Some(0));
    assert_eq!(end.star, Some(0));

    sqlx::query(
        "INSERT INTO user_dungeons
             (user_id, chapter_id, episode_id, star, challenge_count, has_record,
              left_return_all_num, today_pass_num, today_total_num, created_at, updated_at)
             VALUES (18, 9000, 90002501, 2, 0, 0, 1, 0, 0, 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();

    let (update, end) = abort_dungeon_updates(&pool, 18, &active).await.unwrap();

    assert_eq!(update.dungeon_info.unwrap().star, Some(2));
    assert_eq!(end.star, Some(0));
    assert_eq!(end.total_round, Some(15));
}

#[tokio::test]
async fn tutorial_trial_settlement_creates_its_first_progress_row() {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/excel2json");
    let _ = config::init(data_dir.to_str().unwrap());
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at)
         VALUES (28, 'trial-settlement', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    database::db::starter_data::load_all_starter_data(&pool, 28)
        .await
        .unwrap();

    let episode = configs::get().episode.get(110101).unwrap();
    let fight_id = battle_db::create_fight_instance(
        &pool,
        battle_db::NewFightInstance {
            user_id: 28,
            episode_id: episode.id,
            battle_id: episode.battle_id,
            multiplication: 1,
            entry_cost: "{}",
            checkpoint: "{}",
            created_at: 0,
        },
    )
    .await
    .unwrap();
    let active = ActiveBattle {
        fight_id: Some(fight_id),
        chapter_id: episode.chapter_id,
        episode_id: episode.id,
        battle_id: episode.battle_id,
        fight_group: Some(sonettobuf::FightGroup {
            hero_list: vec![-3, -2, -1],
            ..Default::default()
        }),
        ..Default::default()
    };

    let settlement = settle_active(
        &pool,
        28,
        &active,
        DungeonCompletion {
            star: 1,
            total_round: 1,
            multiplier: 1,
            fight_group: active.fight_group.as_ref(),
        },
        &DungeonRecordStatus::default(),
    )
    .await
    .unwrap();

    assert_eq!(
        settlement.dungeon_update.dungeon_info.unwrap().star,
        Some(1)
    );
    assert_eq!(
        dungeons::episode_star(&pool, 28, episode.id).await.unwrap(),
        1
    );
}

#[tokio::test]
async fn unlimited_dungeon_clear_keeps_unrelated_daily_counters() {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/excel2json");
    let _ = config::init(data_dir.to_str().unwrap());
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at) VALUES (19, 'clear', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO user_chapter_type_nums
             (user_id, chapter_type, today_pass_num, today_total_num, last_reset_date)
             VALUES (19, 6, 1, 2, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();

    let (dungeon, counters, _) = dungeons::update_dungeon_progress(&pool, 19, 9040, 90400101, 1)
        .await
        .unwrap();

    assert_eq!(dungeon.challenge_count, 0);
    assert_eq!(dungeon.left_return_all_num, 1);
    assert_eq!((dungeon.today_pass_num, dungeon.today_total_num), (0, 0));
    assert_eq!(counters.len(), 1);
    assert_eq!(counters[0].chapter_type, 6);
    assert_eq!(
        (counters[0].today_pass_num, counters[0].today_total_num),
        (1, 2)
    );
    let (reply, rows) = dungeon_info(&pool, 19).await.unwrap();
    assert_eq!(reply.dungeon_info_size, Some(rows.len() as i32));

    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at) VALUES (20, 'unlock', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    let (unlocked, _, _) = dungeons::unlock_stage(&pool, 20, 90400101).await.unwrap();
    assert!(unlocked.iter().all(|dungeon| {
        dungeon.challenge_count == 0
            && dungeon.left_return_all_num == 1
            && dungeon.today_pass_num == 0
    }));
}

#[test]
fn normal_push_uses_the_runtime_outcome() {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/excel2json");
    let _ = config::init(data_dir.to_str().unwrap());
    let active = ActiveBattle {
        fight_id: Some(42),
        fight_group: Some(Default::default()),
        runtime: battle::engine::runtime::BattleRuntime::new(sonettobuf::Fight {
            attacker: Some(sonettobuf::FightTeam {
                entitys: vec![sonettobuf::FightEntityInfo {
                    uid: Some(1),
                    current_hp: Some(100),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            defender: Some(sonettobuf::FightTeam {
                entitys: vec![sonettobuf::FightEntityInfo {
                    uid: Some(-1),
                    current_hp: Some(0),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    assert_eq!(
        completed_end_fight(&active).record.unwrap().fight_result,
        Some(FightResult::Succ as i32)
    );
}

#[test]
fn episode_first_bonus_uses_bonus_config() {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/excel2json");
    let _ = config::init(data_dir.to_str().unwrap());

    assert_eq!(reward::parse_bonus(1010902).player_cloths, vec![(1, 1)]);
}

#[test]
fn episode_exp_uses_cost_and_player_level_thresholds() {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/excel2json");
    let _ = config::init(data_dir.to_str().unwrap());
    let episode = config::configs::get().episode.get(10101).unwrap();

    assert_eq!(logic::dungeon::episode_player_exp(episode, false, 1), 80);
    assert_eq!(logic::dungeon::episode_player_exp(episode, false, 2), 160);
    assert_eq!(episode_cost(episode, 2).currencies, vec![(4, 16)]);
    assert_eq!(failure_refund(episode, 2).currencies, vec![(4, 16)]);
    assert_eq!(database::db::game::player_infos::level_for_exp(199), 1);
    assert_eq!(database::db::game::player_infos::level_for_exp(200), 2);
    let level_rewards = crate::logic::profile::level_up_rewards(
        database::db::game::player_infos::PlayerLevelChange { from: 1, to: 2 },
    );
    assert_eq!(level_rewards.currencies, vec![(3, 1), (4, 50)]);
}

#[test]
fn dungeon_record_auto_saves_only_new_or_faster_runs() {
    assert!(should_save_record(None, 3));
    assert!(should_save_record(Some(4), 3));
    assert!(should_save_record(Some(3), 3));
    assert!(!should_save_record(Some(2), 3));
}

#[test]
fn battle_star_uses_configured_round_condition() {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/excel2json");
    let _ = config::init(data_dir.to_str().unwrap());
    let runtime = battle::engine::runtime::BattleRuntime::new(sonettobuf::Fight {
        cur_round: Some(3),
        attacker: Some(sonettobuf::FightTeam {
            entitys: vec![sonettobuf::FightEntityInfo {
                uid: Some(1),
                current_hp: Some(100),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    });

    assert_eq!(battle_star(&runtime, 301110), 2);
}

#[tokio::test]
async fn puzzle_progress_belongs_to_an_unlocked_element_and_clears_on_finish() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at) VALUES (13, 'puzzle', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO user_dungeon_elements (user_id, element_id, is_finished)
             VALUES (13, 101, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();

    assert!(
        save_puzzle_progress(&pool, 13, 999, "x".into())
            .await
            .is_err()
    );
    assert_eq!(
        get_puzzle_progress(&pool, 13, 101)
            .await
            .unwrap()
            .progress
            .as_deref(),
        Some("")
    );
    save_puzzle_progress(&pool, 13, 101, "path".into())
        .await
        .unwrap();
    assert_eq!(
        get_puzzle_progress(&pool, 13, 101)
            .await
            .unwrap()
            .progress
            .as_deref(),
        Some("path")
    );
    finish_puzzle(&pool, 13, 101).await.unwrap();
    let saved: String = sqlx::query_scalar(
        "SELECT puzzle_progress FROM user_dungeon_elements
             WHERE user_id = 13 AND element_id = 101",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(saved.is_empty());
    assert_eq!(
        dungeons::get_finished_puzzles(&pool, 13).await.unwrap(),
        vec![101]
    );
}

#[tokio::test]
async fn assist_roster_returns_the_requested_hero_from_other_accounts() {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/excel2json");
    let _ = config::init(data_dir.to_str().unwrap());
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    for (id, name) in [(1_i64, "owner"), (2, "helper")] {
        sqlx::query("INSERT INTO users (id, username, created_at, updated_at) VALUES (?, ?, 0, 0)")
            .bind(id)
            .bind(name)
            .execute(&pool)
            .await
            .unwrap();
        database::db::starter_data::load_all_starter_data(&pool, id)
            .await
            .unwrap();
    }
    database::models::game::heros::UserHeroModel::new(2, pool.clone())
        .create_hero(3023)
        .await
        .unwrap();
    database::db::game::friends::add_friend(&pool, 1, 2)
        .await
        .unwrap();

    let reply = refresh_assist(
        &pool,
        1,
        RefreshAssistRequest {
            assist_type: Some(6),
            ext: Some("3023".into()),
        },
    )
    .await
    .unwrap();

    let candidate = &reply.assist_hero_careers[0].assist_hero_infos[0];
    assert_eq!(candidate.user_id, Some(2));
    assert_eq!(candidate.hero_id, Some(3023));
    assert_eq!(candidate.is_friend, Some(true));
}

#[tokio::test]
async fn settlement_commits_all_or_rolls_back_with_the_active_fight() {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/excel2json");
    let _ = config::init(data_dir.to_str().unwrap());
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at)
             VALUES (23, 'settlement', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    database::db::starter_data::load_all_starter_data(&pool, 23)
        .await
        .unwrap();
    database::models::game::heros::UserHeroModel::new(23, pool.clone())
        .create_hero(3023)
        .await
        .unwrap();

    let episode = configs::get().episode.get(10101).unwrap();
    let chapter_id = episode.chapter_id;
    let fight_id = battle_db::create_fight_instance(
        &pool,
        battle_db::NewFightInstance {
            user_id: 23,
            episode_id: episode.id,
            battle_id: episode.battle_id,
            multiplication: 1,
            entry_cost: "{}",
            checkpoint: "{}",
            created_at: 0,
        },
    )
    .await
    .unwrap();
    let exp_before: i32 = sqlx::query_scalar("SELECT exp FROM users WHERE id = 23")
        .fetch_one(&pool)
        .await
        .unwrap();
    let hero_uid: i64 = sqlx::query_scalar("SELECT uid FROM heroes WHERE user_id = 23 LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    let mut active = ActiveBattle {
        fight_id: Some(fight_id + 1),
        chapter_id,
        episode_id: episode.id,
        battle_id: episode.battle_id,
        fight_group: Some(sonettobuf::FightGroup {
            hero_list: vec![hero_uid],
            ..Default::default()
        }),
        ..Default::default()
    };
    let record = prepare_dungeon_record(&pool, 23, &active, 3).await.unwrap();
    assert!(record.updated);
    assert_eq!(
        dungeons::dungeon_record_round(&pool, 23, episode.id)
            .await
            .unwrap(),
        None
    );
    let result = settle_active(
        &pool,
        23,
        &active,
        DungeonCompletion {
            star: 1,
            total_round: 3,
            multiplier: 1,
            fight_group: active.fight_group.as_ref(),
        },
        &record,
    )
    .await;

    assert!(result.is_err());
    assert_eq!(
        dungeons::episode_star(&pool, 23, episode.id).await.unwrap(),
        0
    );
    let exp_after: i32 = sqlx::query_scalar("SELECT exp FROM users WHERE id = 23")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(exp_after, exp_before);
    assert_eq!(
        dungeons::dungeon_record_round(&pool, 23, episode.id)
            .await
            .unwrap(),
        None
    );
    assert_eq!(
        battle_db::load_active_fight(&pool, 23)
            .await
            .unwrap()
            .unwrap()
            .id,
        fight_id
    );

    let mut faster_record = record.auto_save.clone().unwrap();
    faster_record.round = 2;
    replace_dungeon_record(&pool, 23, &faster_record)
        .await
        .unwrap();
    active.fight_id = Some(fight_id);
    let settlement = settle_active(
        &pool,
        23,
        &active,
        DungeonCompletion {
            star: 1,
            total_round: 3,
            multiplier: 1,
            fight_group: active.fight_group.as_ref(),
        },
        &record,
    )
    .await
    .unwrap();
    assert_eq!(settlement.end_dungeon.update_dungeon_record, Some(false));

    assert_eq!(
        dungeons::episode_star(&pool, 23, episode.id).await.unwrap(),
        1
    );
    let committed_exp: i32 = sqlx::query_scalar("SELECT exp FROM users WHERE id = 23")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(committed_exp > exp_before);
    assert_eq!(
        dungeons::dungeon_record_round(&pool, 23, episode.id)
            .await
            .unwrap(),
        Some(2)
    );
    assert!(
        battle_db::load_active_fight(&pool, 23)
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn battleless_settlement_rolls_back_its_entry_cost_on_failure() {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/excel2json");
    let _ = config::init(data_dir.to_str().unwrap());
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at)
             VALUES (24, 'battleless', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    database::db::starter_data::load_all_starter_data(&pool, 24)
        .await
        .unwrap();
    sqlx::query("UPDATE currencies SET quantity = 100 WHERE user_id = 24 AND currency_id = 4")
        .execute(&pool)
        .await
        .unwrap();
    let episode = configs::get().episode.get(1000002).unwrap();
    let currency_before: i32 = sqlx::query_scalar(
        "SELECT quantity FROM currencies WHERE user_id = 24 AND currency_id = 4",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let result = settle_battleless(
        &pool,
        24,
        -1,
        episode.id,
        DungeonCompletion {
            star: 1,
            total_round: 0,
            multiplier: 1,
            fight_group: None,
        },
        &Default::default(),
    )
    .await;

    assert!(result.is_err());
    let currency_after: i32 = sqlx::query_scalar(
        "SELECT quantity FROM currencies WHERE user_id = 24 AND currency_id = 4",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(currency_after, currency_before);

    settle_battleless(
        &pool,
        24,
        episode.chapter_id,
        episode.id,
        DungeonCompletion {
            star: 1,
            total_round: 0,
            multiplier: 1,
            fight_group: None,
        },
        &Default::default(),
    )
    .await
    .unwrap();
    let currency_after_commit: i32 = sqlx::query_scalar(
        "SELECT quantity FROM currencies WHERE user_id = 24 AND currency_id = 4",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(currency_after_commit, currency_before - 6);

    assert!(
        settle_battleless(
            &pool,
            24,
            episode.chapter_id,
            episode.id,
            DungeonCompletion {
                star: 1,
                total_round: 0,
                multiplier: 1,
                fight_group: None,
            },
            &Default::default(),
        )
        .await
        .is_err()
    );
    let currency_after_retry: i32 = sqlx::query_scalar(
        "SELECT quantity FROM currencies WHERE user_id = 24 AND currency_id = 4",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(currency_after_retry, currency_after_commit);
}

#[tokio::test]
async fn refund_commits_with_active_fight_finalization() {
    let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/excel2json");
    let _ = config::init(data_dir.to_str().unwrap());
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at)
             VALUES (25, 'refund', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    database::db::starter_data::load_all_starter_data(&pool, 25)
        .await
        .unwrap();
    let episode = configs::get().episode.get(10101).unwrap();
    let fight_id = battle_db::create_fight_instance(
        &pool,
        battle_db::NewFightInstance {
            user_id: 25,
            episode_id: episode.id,
            battle_id: episode.battle_id,
            multiplication: 2,
            entry_cost: "{}",
            checkpoint: "{}",
            created_at: 0,
        },
    )
    .await
    .unwrap();
    let currency_before: i32 = sqlx::query_scalar(
        "SELECT quantity FROM currencies WHERE user_id = 25 AND currency_id = 4",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let stored = battle_db::load_active_fight(&pool, 25)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(stored.multiplication, 2);
    let active = ActiveBattle {
        fight_id: Some(fight_id + 1),
        episode_id: episode.id,
        multiplication: Some(stored.multiplication),
        ..Default::default()
    };

    assert!(settle_refund(&pool, 25, &active, false).await.is_err());
    let currency_after_failure: i32 = sqlx::query_scalar(
        "SELECT quantity FROM currencies WHERE user_id = 25 AND currency_id = 4",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(currency_after_failure, currency_before);
    assert!(
        battle_db::load_active_fight(&pool, 25)
            .await
            .unwrap()
            .is_some()
    );

    settle_checkpoint_refund(&pool, 25, fight_id, reward::parse("2#4#16"))
        .await
        .unwrap();
    let currency_after_commit: i32 = sqlx::query_scalar(
        "SELECT quantity FROM currencies WHERE user_id = 25 AND currency_id = 4",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(currency_after_commit, currency_before + 16);
    assert!(
        battle_db::load_active_fight(&pool, 25)
            .await
            .unwrap()
            .is_none()
    );
}
