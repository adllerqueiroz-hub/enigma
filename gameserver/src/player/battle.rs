use crate::error::AppError;
use crate::logic::reward::{self, ConsumedRewards, RewardSet};
use common::time::ServerTime;
use database::db::game::battle;
use flate2::{Compression, read::GzEncoder};
use prost::Message;
use serde::{Deserialize, Serialize};
use sonettobuf::{
    AutoRoundReply, AutoRoundRequest, BeginRoundReply, BeginRoundRequest, CardInfoPush,
    FightReason, FightRoundOperRecord, ReconnectFightReply, RedealCardInfoPush, StartDungeonReply,
    StartDungeonRequest, UseClothSkillOperRecord, UseClothSkillReply, UseClothSkillRequest,
    fight_reason,
};
use sqlx::SqlitePool;
use std::io::Read;

#[derive(Debug, Clone, Default)]
pub struct BattleManager {
    pub active: Option<ActiveBattle>,
    pub pending_record: Option<PendingDungeonRecord>,
}

impl BattleManager {
    pub async fn ensure_can_start(
        &self,
        pool: &SqlitePool,
        player_id: i64,
    ) -> Result<(), AppError> {
        if self.active.is_some() || battle::load_active_fight(pool, player_id).await?.is_some() {
            return Err(AppError::InvalidRequest);
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PendingDungeonRecord {
    pub episode_id: i32,
    pub round: i32,
    pub record: database::db::game::dungeons::PreparedDungeonRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CommittedRound {
    request: BeginRoundRequest,
    cloth_skill_opers: Vec<UseClothSkillOperRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BattleCheckpoint {
    chapter_id: i32,
    start_request: StartDungeonRequest,
    seed: u64,
    tower_context: Option<::battle::tower::BattleContext>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
/// Session and persistence wrapper around the authoritative `BattleRuntime`.
/// Gameplay semantics stay in `battle`; this type supplies inputs and records server metadata.
pub struct ActiveBattle {
    pub tower_type: Option<i32>,
    pub tower_id: Option<i32>,
    pub layer_id: Option<i32>,
    pub episode_id: i32,
    pub chapter_id: i32,
    pub difficulty: Option<i32>,
    pub talent_plan_id: Option<i32>,
    pub team_level: Option<i32>,
    pub assist_boss_level: Option<i32>,
    pub battle_id: i32,
    pub runtime: ::battle::engine::runtime::BattleRuntime,
    pub fight_group: Option<sonettobuf::FightGroup>,
    pub fight_id: Option<i64>,
    pub is_replay: Option<bool>,
    pub replay_episode_id: Option<i32>,
    pub multiplication: Option<i32>,
    pub params: Option<String>,
    pub ai_deck: Vec<sonettobuf::CardInfo>,
    pub(crate) seed: u64,
    pub(crate) start_request: Option<StartDungeonRequest>,
    pub(crate) tower_context: Option<::battle::tower::BattleContext>,
    pub(crate) rounds: Vec<CommittedRound>,
    pub(crate) pending_cloth_skill_opers: Vec<UseClothSkillOperRecord>,
}

impl ActiveBattle {
    pub fn plan_auto_round(&self, request: &AutoRoundRequest) -> AutoRoundReply {
        self.runtime.plan_auto_round(request)
    }

    pub fn use_cloth_skill(
        &mut self,
        request: UseClothSkillRequest,
    ) -> Result<(UseClothSkillReply, Option<RedealCardInfoPush>), AppError> {
        let reply = self
            .runtime
            .use_cloth_skill(request)
            .ok_or(AppError::InvalidRequest)?;
        self.pending_cloth_skill_opers
            .push(UseClothSkillOperRecord {
                skill_id: request.skill_id,
                from_id: request.from_id,
                to_id: request.to_id,
                r#type: request.r#type,
            });
        Ok((reply, self.runtime.take_redeal_card_push()))
    }

    pub async fn prepare(
        pool: &SqlitePool,
        player_id: i64,
        episode_id: i32,
        battle_id: i32,
        request: StartDungeonRequest,
    ) -> Result<Self, AppError> {
        let use_record = request.use_record.unwrap_or(false);
        let fight_group = request
            .fight_group
            .as_ref()
            .ok_or(AppError::InvalidRequest)?;
        let built = ::battle::dungeon::build_fight(
            pool,
            player_id,
            episode_id,
            battle_id,
            use_record,
            fight_group,
            request.params.as_deref(),
        )
        .await?;
        Self::prepare_from_built(request, built, None)
    }

    pub async fn from_built(
        pool: &SqlitePool,
        player_id: i64,
        request: StartDungeonRequest,
        built: ::battle::dungeon::BuiltFight,
        tower_context: Option<::battle::tower::BattleContext>,
    ) -> Result<Self, AppError> {
        let mut active = Self::prepare_from_built(request, built, tower_context)?;
        let checkpoint = active.checkpoint_json()?;
        active.fight_id = Some(
            battle::create_fight_instance(
                pool,
                battle::NewFightInstance {
                    user_id: player_id,
                    episode_id: active.episode_id,
                    battle_id: active.battle_id,
                    multiplication: active.multiplication.unwrap_or(1).max(1),
                    entry_cost: "{}",
                    checkpoint: &checkpoint,
                    created_at: ServerTime::now_ms(),
                },
            )
            .await?,
        );
        Ok(active)
    }

    fn prepare_from_built(
        request: StartDungeonRequest,
        built: ::battle::dungeon::BuiltFight,
        tower_context: Option<::battle::tower::BattleContext>,
    ) -> Result<Self, AppError> {
        let episode_id = request.episode_id.ok_or(AppError::InvalidRequest)?;
        let chapter_id = request.chapter_id.unwrap_or_else(|| {
            config::configs::get()
                .episode
                .get(episode_id)
                .map(|episode| episode.chapter_id)
                .unwrap_or_default()
        });
        let battle_id = built
            .fight
            .battle_id
            .filter(|battle_id| *battle_id > 0)
            .ok_or(AppError::InvalidRequest)?;
        let seed = rand::random();
        Self::from_built_with_seed(
            chapter_id,
            episode_id,
            battle_id,
            request,
            built,
            tower_context,
            seed,
        )
    }

    pub async fn activate(
        &mut self,
        pool: &SqlitePool,
        player_id: i64,
        costs: &RewardSet,
    ) -> Result<ConsumedRewards, AppError> {
        if self.fight_id.is_some() {
            return Err(AppError::InvalidRequest);
        }

        let checkpoint = self.checkpoint_json()?;
        let entry_cost = serde_json::to_string(costs)?;
        let mut tx = pool.begin().await?;
        let consumed = reward::RewardManager::new(player_id)
            .consume(&mut tx, costs)
            .await?;
        let fight_id = battle::create_fight_instance_in_transaction(
            &mut tx,
            battle::NewFightInstance {
                user_id: player_id,
                episode_id: self.episode_id,
                battle_id: self.battle_id,
                multiplication: self.multiplication.unwrap_or(1).max(1),
                entry_cost: &entry_cost,
                checkpoint: &checkpoint,
                created_at: ServerTime::now_ms(),
            },
        )
        .await?;
        tx.commit().await?;
        self.fight_id = Some(fight_id);
        Ok(consumed)
    }

    fn from_built_with_seed(
        chapter_id: i32,
        episode_id: i32,
        battle_id: i32,
        request: StartDungeonRequest,
        built: ::battle::dungeon::BuiltFight,
        tower_context: Option<::battle::tower::BattleContext>,
        seed: u64,
    ) -> Result<Self, AppError> {
        let use_record = request.use_record.unwrap_or(false);
        let fight_group = request
            .fight_group
            .clone()
            .ok_or(AppError::InvalidRequest)?;
        let attacker = built.fight.attacker.as_ref();
        let team_level = attacker.and_then(average_team_level);
        let assist_boss_level = attacker
            .and_then(|team| team.assist_boss.as_ref())
            .and_then(|boss| boss.level);
        let mut runtime = ::battle::engine::runtime::BattleRuntime::new_with_attributes(
            built.fight,
            built.ex_attributes,
            built.sp_attributes,
        );
        runtime.extend_battle_rule_skills(built.battle_rule_skills);
        runtime
            .start_round_with_determinism(
                ::battle::engine::runtime::determinism::RoundDeterminism::with_seed(seed),
            )
            .map_err(AppError::Custom)?;

        Ok(Self {
            tower_type: tower_context.map(|context| context.tower_type),
            tower_id: tower_context.map(|context| context.tower_id),
            layer_id: tower_context.map(|context| context.layer_id),
            difficulty: tower_context.map(|context| context.difficulty),
            talent_plan_id: tower_context.map(|context| context.talent_plan_id),
            episode_id,
            chapter_id,
            battle_id,
            runtime,
            fight_group: Some(fight_group),
            fight_id: None,
            is_replay: Some(use_record),
            multiplication: request.multiplication,
            params: request.params.clone(),
            team_level,
            assist_boss_level,
            seed,
            start_request: Some(request),
            tower_context,
            ..Default::default()
        })
    }

    pub async fn restore(
        pool: &SqlitePool,
        player_id: i64,
        record: database::db::game::battle::ActiveFightRecord,
    ) -> Result<Self, AppError> {
        let checkpoint: BattleCheckpoint = serde_json::from_str(&record.checkpoint)
            .map_err(|error| AppError::InvalidBattleCheckpoint(error.to_string()))?;
        let episode_id = checkpoint.start_request.episode_id.ok_or_else(|| {
            AppError::InvalidBattleCheckpoint("start request has no episode".into())
        })?;
        if episode_id != record.episode_id {
            return Err(AppError::InvalidBattleCheckpoint(
                "checkpoint episode does not match fight instance".into(),
            ));
        }
        let fight_group = checkpoint
            .start_request
            .fight_group
            .as_ref()
            .ok_or_else(|| {
                AppError::InvalidBattleCheckpoint("start request has no fight group".into())
            })?;
        let use_record = checkpoint.start_request.use_record.unwrap_or(false);
        let built = if let Some(context) = checkpoint.tower_context {
            ::battle::tower::build_fight(
                pool,
                player_id,
                episode_id,
                record.battle_id,
                use_record,
                fight_group,
                context,
            )
            .await?
        } else {
            ::battle::dungeon::build_fight(
                pool,
                player_id,
                episode_id,
                record.battle_id,
                use_record,
                fight_group,
                checkpoint.start_request.params.as_deref(),
            )
            .await?
        };
        let mut active = Self::from_built_with_seed(
            checkpoint.chapter_id,
            episode_id,
            record.battle_id,
            checkpoint.start_request,
            built,
            checkpoint.tower_context,
            checkpoint.seed,
        )?;
        active.fight_id = Some(record.id);
        Ok(active)
    }

    pub fn checkpoint_json(&self) -> Result<String, AppError> {
        Ok(serde_json::to_string(&BattleCheckpoint {
            chapter_id: self.chapter_id,
            start_request: self.start_request.clone().ok_or(AppError::InvalidRequest)?,
            seed: self.seed,
            tower_context: self.tower_context,
        })?)
    }

    pub fn reconnect_reply(&self) -> ReconnectFightReply {
        let (fight, last_round) = self.runtime.reconnect_state();
        ReconnectFightReply {
            fight: Some(fight),
            last_round,
            fight_reason: Some(FightReason {
                r#type: Some(if self.is_replay.unwrap_or(false) {
                    fight_reason::FightType::DungeonRecord as i32
                } else {
                    fight_reason::FightType::Dungeon as i32
                }),
                content: Some(self.episode_id.to_string()),
                battle_id: Some(self.battle_id),
                multiplication: self.multiplication,
                data: self.params.clone(),
            }),
            fight_group: self.fight_group.clone(),
        }
    }

    pub fn oper_records(&self) -> Vec<FightRoundOperRecord> {
        self.rounds
            .iter()
            .map(|round| FightRoundOperRecord {
                cloth_skill_opers: round.cloth_skill_opers.clone(),
                opers: round.request.opers.clone(),
            })
            .collect()
    }

    pub fn start_reply(&self) -> StartDungeonReply {
        ::battle::dungeon::start_reply(&self.runtime)
    }

    pub fn card_info_push(&self) -> CardInfoPush {
        self.runtime.card_info_push()
    }

    pub fn begin_round(&mut self, request: BeginRoundRequest) -> Result<BeginRoundReply, AppError> {
        let reply = ::battle::dungeon::begin_round(&mut self.runtime, request.clone())
            .map_err(AppError::Custom)?;
        self.record_round(request);
        compress_round_steps(reply)
    }

    fn record_round(&mut self, request: BeginRoundRequest) {
        self.rounds.push(CommittedRound {
            request,
            cloth_skill_opers: std::mem::take(&mut self.pending_cloth_skill_opers),
        });
    }
}

fn average_team_level(team: &sonettobuf::FightTeam) -> Option<i32> {
    let (sum, count) = team
        .entitys
        .iter()
        .filter_map(|entity| entity.level)
        .fold((0, 0), |(sum, count), level| (sum + level, count + 1));
    (count > 0).then(|| sum / count)
}

/// Applies client transport framing after the battle reply is complete.
/// Framing may encode and compress steps but never filter, reorder, or synthesize them.
fn compress_round_steps(mut reply: BeginRoundReply) -> Result<BeginRoundReply, AppError> {
    let Some(round) = reply.round.as_mut() else {
        return Ok(reply);
    };
    let step_count = i32::try_from(round.fight_step.len())
        .map_err(|_| AppError::Custom("fight step count exceeds i32".to_owned()))?;
    let mut framed = Vec::new();
    framed.extend_from_slice(&step_count.to_be_bytes());
    for step in &round.fight_step {
        let bytes = step.encode_to_vec();
        let len = i32::try_from(bytes.len())
            .map_err(|_| AppError::Custom("encoded fight step exceeds i32".to_owned()))?;
        framed.extend_from_slice(&len.to_be_bytes());
        framed.extend_from_slice(&bytes);
    }

    let mut encoder = GzEncoder::new(framed.as_slice(), Compression::default());
    let mut compressed = Vec::new();
    encoder.read_to_end(&mut compressed)?;
    round.total_step = Some(step_count);
    round.fight_step_bytes = Some(compressed);
    round.fight_step.clear();
    Ok(reply)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::read::GzDecoder;

    #[test]
    fn team_level_is_absent_when_no_entity_has_a_level() {
        let empty = sonettobuf::FightTeam::default();
        assert_eq!(average_team_level(&empty), None);

        let team = sonettobuf::FightTeam {
            entitys: vec![
                sonettobuf::FightEntityInfo {
                    level: Some(10),
                    ..Default::default()
                },
                sonettobuf::FightEntityInfo::default(),
                sonettobuf::FightEntityInfo {
                    level: Some(20),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_eq!(average_team_level(&team), Some(15));
    }

    #[test]
    fn replay_record_keeps_cloth_and_card_operations_in_the_same_round() {
        let mut active = ActiveBattle::default();
        active
            .pending_cloth_skill_opers
            .push(UseClothSkillOperRecord {
                skill_id: Some(12),
                ..Default::default()
            });

        active.record_round(BeginRoundRequest {
            opers: vec![sonettobuf::BeginRoundOper::default()],
            ..Default::default()
        });

        let records = active.oper_records();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].cloth_skill_opers[0].skill_id, Some(12));
        assert_eq!(records[0].opers.len(), 1);
        assert!(active.pending_cloth_skill_opers.is_empty());
    }

    #[test]
    fn begin_round_steps_use_the_clients_compressed_framing() {
        let reply = compress_round_steps(BeginRoundReply {
            round: Some(sonettobuf::FightRound {
                fight_step: vec![Default::default(), Default::default()],
                ..Default::default()
            }),
        })
        .unwrap();
        let round = reply.round.unwrap();

        assert_eq!(round.total_step, Some(2));
        assert!(round.fight_step.is_empty());

        let compressed = round.fight_step_bytes.unwrap();
        let mut decoder = GzDecoder::new(compressed.as_slice());
        let mut framed = Vec::new();
        decoder.read_to_end(&mut framed).unwrap();
        assert_eq!(framed, [0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn active_fight_reconnects_from_its_fresh_start_checkpoint() {
        std::thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
                        let _ = config::init(&data_dir);
                        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
                        database::run_migrations(&pool).await.unwrap();
                        sqlx::query(
                            "INSERT INTO users (id, username, created_at, updated_at)
                             VALUES (9, 'reconnect', 0, 0)",
                        )
                        .execute(&pool)
                        .await
                        .unwrap();
                        database::db::starter_data::load_all_starter_data(&pool, 9)
                            .await
                            .unwrap();

                        let mut active = ActiveBattle::prepare(
                            &pool,
                            9,
                            10002,
                            1002,
                            StartDungeonRequest {
                                chapter_id: Some(301),
                                episode_id: Some(10002),
                                fight_group: Some(Default::default()),
                                multiplication: Some(1),
                                ..Default::default()
                            },
                        )
                        .await
                        .unwrap();
                        active
                            .activate(&pool, 9, &RewardSet::default())
                            .await
                            .unwrap();
                        let expected = active.reconnect_reply();
                        let expected_start = active.start_reply();
                        let expected_cards = active.card_info_push();
                        active.begin_round(BeginRoundRequest::default()).unwrap();
                        let fight_id = active.fight_id.unwrap();
                        assert!(matches!(
                            BattleManager::default().ensure_can_start(&pool, 9).await,
                            Err(AppError::InvalidRequest)
                        ));
                        assert!(
                            battle::create_fight_instance(
                                &pool,
                                battle::NewFightInstance {
                                    user_id: 9,
                                    episode_id: 10002,
                                    battle_id: 1002,
                                    multiplication: 1,
                                    entry_cost: "{}",
                                    checkpoint: "{}",
                                    created_at: 0,
                                },
                            )
                            .await
                            .is_err()
                        );

                        let record = battle::load_active_fight(&pool, 9).await.unwrap().unwrap();
                        let restored = ActiveBattle::restore(&pool, 9, record).await.unwrap();

                        assert_eq!(restored.reconnect_reply(), expected);
                        assert_eq!(restored.start_reply(), expected_start);
                        assert_eq!(restored.card_info_push(), expected_cards);
                        assert!(restored.oper_records().is_empty());
                        battle::finish_fight_instance(&pool, 9, fight_id)
                            .await
                            .unwrap();
                        assert!(battle::load_active_fight(&pool, 9).await.unwrap().is_none());
                        BattleManager::default()
                            .ensure_can_start(&pool, 9)
                            .await
                            .unwrap();
                    });
            })
            .unwrap()
            .join()
            .unwrap();
    }

    #[tokio::test]
    async fn malformed_checkpoint_is_the_only_discardable_restore_error() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let error = ActiveBattle::restore(
            &pool,
            9,
            database::db::game::battle::ActiveFightRecord {
                id: 1,
                episode_id: 10002,
                battle_id: 1002,
                multiplication: 1,
                entry_cost: "{}".into(),
                checkpoint: "{".into(),
            },
        )
        .await
        .unwrap_err();

        assert!(matches!(error, AppError::InvalidBattleCheckpoint(_)));
    }

    #[tokio::test]
    async fn activation_rolls_back_cost_when_an_active_fight_already_exists() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        database::run_migrations(&pool).await.unwrap();
        sqlx::query(
            "INSERT INTO users (id, username, created_at, updated_at)
             VALUES (9, 'activation', 0, 0)",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO currencies (user_id, currency_id, quantity)
             VALUES (9, 4, 5)",
        )
        .execute(&pool)
        .await
        .unwrap();
        let existing_fight = battle::create_fight_instance(
            &pool,
            battle::NewFightInstance {
                user_id: 9,
                episode_id: 10002,
                battle_id: 1002,
                multiplication: 1,
                entry_cost: "{}",
                checkpoint: "{}",
                created_at: 0,
            },
        )
        .await
        .unwrap();

        let mut active = ActiveBattle {
            chapter_id: 301,
            episode_id: 10002,
            battle_id: 1002,
            seed: 7,
            start_request: Some(StartDungeonRequest {
                chapter_id: Some(301),
                episode_id: Some(10002),
                fight_group: Some(Default::default()),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(
            active
                .activate(&pool, 9, &reward::parse("2#4#3"))
                .await
                .is_err()
        );

        let quantity: i32 = sqlx::query_scalar(
            "SELECT quantity FROM currencies WHERE user_id = 9 AND currency_id = 4",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(quantity, 5);
        assert!(active.fight_id.is_none());

        battle::finish_fight_instance(&pool, 9, existing_fight)
            .await
            .unwrap();
        active
            .activate(&pool, 9, &reward::parse("2#4#3"))
            .await
            .unwrap();
        let record = battle::load_active_fight(&pool, 9).await.unwrap().unwrap();
        let entry_cost: RewardSet = serde_json::from_str(&record.entry_cost).unwrap();
        assert_eq!(entry_cost.currencies, vec![(4, 3)]);
    }
}
