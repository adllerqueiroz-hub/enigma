use crate::models::game::rouge::{
    Rouge2CareerLevel, Rouge2MaterialState, Rouge2RewardState, Rouge2State,
};
use anyhow::Result;
use sqlx::SqlitePool;

pub struct Rouge2OutsideState {
    pub state: Rouge2State,
    pub career_levels: Vec<Rouge2CareerLevel>,
    pub rewards: Vec<Rouge2RewardState>,
    pub materials: Vec<Rouge2MaterialState>,
}

#[derive(Clone, Copy)]
pub enum Rouge2UnlockKind {
    Relic,
    Buff,
    ActiveSkill,
}

impl Rouge2UnlockKind {
    const fn id(self) -> i32 {
        match self {
            Self::Relic => 1,
            Self::Buff => 2,
            Self::ActiveSkill => 3,
        }
    }
}

pub async fn get_or_create_rouge2_outside(
    pool: &SqlitePool,
    user_id: i64,
    tables: &config::GameDB,
) -> Result<Rouge2OutsideState> {
    let now = common::time::ServerTime::now_ms();

    sqlx::query(
        "INSERT INTO user_rouge2_state (user_id, updated_at)
         VALUES (?, ?)
         ON CONFLICT DO NOTHING",
    )
    .bind(user_id)
    .bind(now)
    .execute(pool)
    .await?;

    for career in tables.rouge2_career.iter() {
        sqlx::query(
            "INSERT INTO user_rouge2_career_levels (user_id, career_id, updated_at)
             VALUES (?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(career.id)
        .bind(now)
        .execute(pool)
        .await?;
    }

    for reward in tables.rouge2_reward.iter() {
        sqlx::query(
            "INSERT INTO user_rouge2_rewards (user_id, reward_id, updated_at)
             VALUES (?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(reward.id)
        .bind(now)
        .execute(pool)
        .await?;
    }

    for material in tables.rouge2_material.iter() {
        sqlx::query(
            "INSERT INTO user_rouge2_materials (user_id, material_id, updated_at)
             VALUES (?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(material.id)
        .bind(now)
        .execute(pool)
        .await?;
    }

    let state = sqlx::query_as::<_, Rouge2State>(
        "SELECT user_id, state, difficulty, coin, end_id, game_num,
                genius_point, genius_ids, reward_point, max_difficulty,
                pass_layer_ids, pass_event_ids, pass_end_ids, pass_entrust_ids,
                pass_collections, last_game_time, hotfix_str, updated_at
         FROM user_rouge2_state
         WHERE user_id = ?",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let career_levels = sqlx::query_as::<_, Rouge2CareerLevel>(
        "SELECT user_id, career_id, exp, updated_at
         FROM user_rouge2_career_levels
         WHERE user_id = ?
         ORDER BY career_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let rewards = sqlx::query_as::<_, Rouge2RewardState>(
        "SELECT user_id, reward_id, buy_count, updated_at
         FROM user_rouge2_rewards
         WHERE user_id = ?
         ORDER BY reward_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let materials = sqlx::query_as::<_, Rouge2MaterialState>(
        "SELECT user_id, material_id, num, updated_at
         FROM user_rouge2_materials
         WHERE user_id = ?
         ORDER BY material_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(Rouge2OutsideState {
        state,
        career_levels,
        rewards,
        materials,
    })
}

pub async fn get_or_create_unlock_ids(
    pool: &SqlitePool,
    user_id: i64,
    tables: &config::GameDB,
    kind: Rouge2UnlockKind,
) -> Result<Vec<i32>> {
    let now = common::time::ServerTime::now_ms();
    let ids: Vec<i32> = match kind {
        Rouge2UnlockKind::Relic => tables.rouge2_relics.iter().map(|row| row.id).collect(),
        Rouge2UnlockKind::Buff => tables.rouge2_buff.iter().map(|row| row.id).collect(),
        Rouge2UnlockKind::ActiveSkill => tables
            .rouge2_active_skill
            .iter()
            .map(|row| row.id)
            .collect(),
    };

    for id in ids {
        sqlx::query(
            "INSERT INTO user_rouge2_unlocks (user_id, unlock_type, unlock_id, updated_at)
             VALUES (?, ?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(kind.id())
        .bind(id)
        .bind(now)
        .execute(pool)
        .await?;
    }

    Ok(sqlx::query_scalar(
        "SELECT unlock_id
         FROM user_rouge2_unlocks
         WHERE user_id = ? AND unlock_type = ?
         ORDER BY unlock_id",
    )
    .bind(user_id)
    .bind(kind.id())
    .fetch_all(pool)
    .await?)
}
