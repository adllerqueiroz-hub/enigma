use crate::models::game::tower::*;
use anyhow::Result;
use common::time::ServerTime;
use sonettobuf;
use sqlx::SqlitePool;
use std::collections::BTreeMap;

pub async fn get_tower_info(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<(
    UserTowerInfo,
    Vec<TowerOpen>,
    Vec<sonettobuf::TowerNo>,
    Vec<AssistBossInfo>,
)> {
    // Get main info
    let mut info =
        sqlx::query_as::<_, UserTowerInfo>("SELECT * FROM user_tower_info WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await?
            .unwrap_or(UserTowerInfo {
                user_id,
                mop_up_times: 0,
                trial_hero_season: 0,
            });
    info.trial_hero_season =
        current_trial_hero_season(config::configs::get(), ServerTime::now_ms());

    let tower_opens = tower_open_schedule(config::configs::get(), ServerTime::now_ms());

    // Get towers
    let towers = get_towers(pool, user_id).await?;

    // Get assist bosses
    let assist_bosses = get_assist_bosses(pool, user_id).await?;

    Ok((info, tower_opens, towers, assist_bosses))
}

pub fn tower_open_schedule(tables: &config::GameDB, now: i64) -> Vec<TowerOpen> {
    let mut boss_rounds = BTreeMap::new();
    for row in tables
        .tower_boss_time
        .iter()
        .filter(|row| row.is_online != 0)
    {
        if let Some(open) = tower_open(
            now,
            TowerType::Boss,
            row.tower_id,
            row.round,
            &row.start_time,
            &row.end_time,
            &row.task_end_time,
        ) {
            boss_rounds
                .entry(row.tower_id)
                .and_modify(|current: &mut TowerOpen| {
                    if open.status == TowerStatus::Open.id()
                        || open.tower_start_time < current.tower_start_time
                    {
                        *current = open.clone();
                    }
                })
                .or_insert(open);
        }
    }

    let mut opens = boss_rounds.into_values().collect::<Vec<_>>();
    let mut limited = tables
        .tower_limited_time
        .iter()
        .filter(|row| row.is_online != 0)
        .filter_map(|row| {
            tower_open(
                now,
                TowerType::Limited,
                row.season,
                0,
                &row.start_time,
                &row.end_time,
                &row.end_time,
            )
        })
        .collect::<Vec<_>>();
    limited.sort_by_key(|open| open.tower_start_time);
    opens.extend(limited.into_iter().take(4));
    opens.sort_by_key(|open| (open.tower_type, open.tower_id));
    opens
}

pub fn current_trial_hero_season(tables: &config::GameDB, now: i64) -> i32 {
    tables
        .tower_hero_trial
        .iter()
        .filter(|row| is_open(now, &row.start_time, &row.end_time))
        .map(|row| row.season)
        .max()
        .unwrap_or_default()
}

fn tower_open(
    now: i64,
    tower_type: TowerType,
    tower_id: i32,
    round: i32,
    start: &str,
    end: &str,
    task_end: &str,
) -> Option<TowerOpen> {
    let tower_start_time = ServerTime::config_date_start_ms(start)?;
    let next_time = ServerTime::config_date_end_ms(end).unwrap_or_default();
    if next_time != 0 && now > next_time {
        return None;
    }
    Some(TowerOpen {
        tower_type: tower_type.id(),
        tower_id,
        status: if now >= tower_start_time {
            TowerStatus::Open.id()
        } else {
            TowerStatus::Ready.id()
        },
        round,
        next_time,
        tower_start_time,
        task_end_time: ServerTime::config_date_end_ms(task_end).unwrap_or_default(),
    })
}

fn is_open(now: i64, start: &str, end: &str) -> bool {
    ServerTime::config_date_start_ms(start).is_some_and(|start| start <= now)
        && ServerTime::config_date_end_ms(end).is_none_or(|end| now <= end)
}

async fn get_towers(pool: &SqlitePool, user_id: i64) -> Result<Vec<sonettobuf::TowerNo>> {
    let tower_data: Vec<(i32, i32, i32, i32, String)> = sqlx::query_as(
        "SELECT tower_type, tower_id, pass_layer_id, history_high_score, params
         FROM user_towers WHERE user_id = ? ORDER BY tower_type, tower_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut towers = Vec::new();
    for (tower_type, tower_id, pass_layer_id, history_high_score, params) in tower_data {
        // Get layers
        let layer_nos = get_tower_layers(pool, user_id, tower_type, tower_id).await?;

        // Get open special layer IDs
        let open_sp_layer_ids = sqlx::query_scalar(
            "SELECT sp_layer_id FROM user_tower_open_sp_layers WHERE user_id = ? AND tower_type = ? AND tower_id = ?"
        )
        .bind(user_id)
        .bind(tower_type)
        .bind(tower_id)
        .fetch_all(pool)
        .await?;

        // Get pass teach IDs
        let pass_teach_ids = sqlx::query_scalar(
            "SELECT teach_id FROM user_tower_pass_teaches WHERE user_id = ? AND tower_type = ? AND tower_id = ?"
        )
        .bind(user_id)
        .bind(tower_type)
        .bind(tower_id)
        .fetch_all(pool)
        .await?;

        towers.push(sonettobuf::TowerNo {
            r#type: Some(tower_type),
            tower_id: Some(tower_id),
            pass_layer_id: Some(pass_layer_id),
            layer_n_os: layer_nos,
            open_sp_layer_ids,
            history_high_score: Some(history_high_score),
            params: (!params.is_empty()).then_some(params),
            pass_teach_ids,
        });
    }

    Ok(towers)
}

async fn get_tower_layers(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
) -> Result<Vec<sonettobuf::LayerNo>> {
    let layers: Vec<(i32, i32, i32)> = sqlx::query_as(
        "SELECT layer_id, curr_high_score, history_high_score
         FROM user_tower_layers WHERE user_id = ? AND tower_type = ? AND tower_id = ?
         ORDER BY layer_id",
    )
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .fetch_all(pool)
    .await?;

    let mut layer_nos = Vec::new();
    for (layer_id, curr_high_score, history_high_score) in layers {
        let episode_nos = get_layer_episodes(pool, user_id, tower_type, tower_id, layer_id).await?;

        layer_nos.push(sonettobuf::LayerNo {
            layer_id: Some(layer_id),
            curr_high_score: Some(curr_high_score),
            history_high_score: Some(history_high_score),
            episode_n_os: episode_nos,
        });
    }

    Ok(layer_nos)
}

async fn get_layer_episodes(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
    layer_id: i32,
) -> Result<Vec<sonettobuf::EpisodeNo>> {
    let episodes: Vec<(i32, i32, i32)> = sqlx::query_as(
        "SELECT episode_id, status, assist_boss_id
         FROM user_tower_episodes
         WHERE user_id = ? AND tower_type = ? AND tower_id = ? AND layer_id = ?
         ORDER BY episode_id",
    )
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .bind(layer_id)
    .fetch_all(pool)
    .await?;

    let mut episode_nos = Vec::new();
    for (episode_id, status, assist_boss_id) in episodes {
        let heroes =
            get_episode_heroes(pool, user_id, tower_type, tower_id, layer_id, episode_id).await?;

        episode_nos.push(sonettobuf::EpisodeNo {
            episode_id: Some(episode_id),
            status: Some(status),
            heros: heroes.into_iter().map(Into::into).collect(),
            assist_boss_id: Some(assist_boss_id),
        });
    }

    Ok(episode_nos)
}

async fn get_episode_heroes(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
    layer_id: i32,
    episode_id: i32,
) -> Result<Vec<HeroInfo>> {
    let hero_data: Vec<(i32, i32)> = sqlx::query_as(
        "SELECT hero_id, trial_id FROM user_tower_episode_heroes
         WHERE user_id = ? AND tower_type = ? AND tower_id = ? AND layer_id = ? AND episode_id = ?",
    )
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .bind(layer_id)
    .bind(episode_id)
    .fetch_all(pool)
    .await?;

    let mut heroes = Vec::new();
    for (hero_id, trial_id) in hero_data {
        let equip_uids = sqlx::query_scalar(
            "SELECT equip_uid FROM user_tower_episode_hero_equips
             WHERE user_id = ? AND tower_type = ? AND tower_id = ? AND layer_id = ? AND episode_id = ? AND hero_id = ?"
        )
        .bind(user_id)
        .bind(tower_type)
        .bind(tower_id)
        .bind(layer_id)
        .bind(episode_id)
        .bind(hero_id)
        .fetch_all(pool)
        .await?;

        heroes.push(HeroInfo {
            hero_id,
            equip_uids,
            trial_id,
        });
    }

    Ok(heroes)
}

async fn get_assist_bosses(pool: &SqlitePool, user_id: i64) -> Result<Vec<AssistBossInfo>> {
    let bosses: Vec<(i32, i32, i32)> = sqlx::query_as(
        "SELECT boss_id, level, use_talent_plan FROM user_assist_bosses WHERE user_id = ? ORDER BY boss_id"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut assist_bosses = Vec::new();
    for (boss_id, level, use_talent_plan) in bosses {
        // Get talent plans
        let plan_data: Vec<(i32, i32, String)> = sqlx::query_as(
            "SELECT plan_id, talent_point, plan_name
             FROM user_assist_boss_talent_plans WHERE user_id = ? AND boss_id = ? ORDER BY plan_id",
        )
        .bind(user_id)
        .bind(boss_id)
        .fetch_all(pool)
        .await?;

        let mut talent_plans = Vec::new();
        for (plan_id, talent_point, plan_name) in plan_data {
            let talent_ids = sqlx::query_scalar(
                "SELECT talent_id FROM user_assist_boss_plan_talents WHERE user_id = ? AND boss_id = ? AND plan_id = ?"
            )
            .bind(user_id)
            .bind(boss_id)
            .bind(plan_id)
            .fetch_all(pool)
            .await?;

            talent_plans.push(TalentPlanInfo {
                plan_id,
                talent_point,
                talent_ids,
                plan_name,
            });
        }

        assist_bosses.push(AssistBossInfo {
            boss_id,
            level,
            talent_plans,
            use_talent_plan,
        });
    }

    Ok(assist_bosses)
}

pub async fn active_talent_plan(
    pool: &SqlitePool,
    user_id: i64,
    boss_id: i32,
) -> sqlx::Result<Option<ActiveTalentPlan>> {
    let Some((boss_level, plan_id, talent_point)) = sqlx::query_as::<_, (i32, i32, i32)>(
        "SELECT boss.level, boss.use_talent_plan, plan.talent_point
         FROM user_assist_bosses boss
         JOIN user_assist_boss_talent_plans plan
           ON plan.user_id = boss.user_id
          AND plan.boss_id = boss.boss_id
          AND plan.plan_id = boss.use_talent_plan
         WHERE boss.user_id = ? AND boss.boss_id = ?",
    )
    .bind(user_id)
    .bind(boss_id)
    .fetch_optional(pool)
    .await?
    else {
        return Ok(None);
    };
    let talent_ids = sqlx::query_scalar(
        "SELECT talent_id FROM user_assist_boss_plan_talents
         WHERE user_id = ? AND boss_id = ? AND plan_id = ?",
    )
    .bind(user_id)
    .bind(boss_id)
    .bind(plan_id)
    .fetch_all(pool)
    .await?;

    Ok(Some(ActiveTalentPlan {
        boss_level,
        plan_id,
        talent_point,
        talent_ids,
    }))
}

pub async fn assist_boss_level(
    pool: &SqlitePool,
    user_id: i64,
    boss_id: i32,
) -> sqlx::Result<Option<i32>> {
    sqlx::query_scalar("SELECT level FROM user_assist_bosses WHERE user_id = ? AND boss_id = ?")
        .bind(user_id)
        .bind(boss_id)
        .fetch_optional(pool)
        .await
}

pub async fn talent_plan_ids(
    pool: &SqlitePool,
    user_id: i64,
    boss_id: i32,
    plan_id: i32,
) -> sqlx::Result<Vec<i32>> {
    sqlx::query_scalar(
        "SELECT talent_id FROM user_assist_boss_plan_talents
         WHERE user_id = ? AND boss_id = ? AND plan_id = ? ORDER BY talent_id",
    )
    .bind(user_id)
    .bind(boss_id)
    .bind(plan_id)
    .fetch_all(pool)
    .await
}

pub async fn consume_mop_up_times(
    pool: &SqlitePool,
    user_id: i64,
    times: i32,
) -> sqlx::Result<Option<i32>> {
    let result = sqlx::query(
        "UPDATE user_tower_info SET mop_up_times = mop_up_times - ?
         WHERE user_id = ? AND ? > 0 AND mop_up_times >= ?",
    )
    .bind(times)
    .bind(user_id)
    .bind(times)
    .bind(times)
    .execute(pool)
    .await?;
    if result.rows_affected() != 1 {
        return Ok(None);
    }
    sqlx::query_scalar("SELECT mop_up_times FROM user_tower_info WHERE user_id = ?")
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

pub async fn consume_mop_up_times_in_transaction(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    user_id: i64,
    times: i32,
) -> sqlx::Result<Option<i32>> {
    let result = sqlx::query(
        "UPDATE user_tower_info SET mop_up_times = mop_up_times - ?
         WHERE user_id = ? AND ? > 0 AND mop_up_times >= ?",
    )
    .bind(times)
    .bind(user_id)
    .bind(times)
    .bind(times)
    .execute(&mut **tx)
    .await?;
    if result.rows_affected() != 1 {
        return Ok(None);
    }
    sqlx::query_scalar("SELECT mop_up_times FROM user_tower_info WHERE user_id = ?")
        .bind(user_id)
        .fetch_optional(&mut **tx)
        .await
}

pub async fn tower_pass_layer(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
) -> sqlx::Result<Option<i32>> {
    sqlx::query_scalar(
        "SELECT pass_layer_id FROM user_towers
         WHERE user_id = ? AND tower_type = ? AND tower_id = ?",
    )
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .fetch_optional(pool)
    .await
}

pub async fn reset_sub_episode(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
    layer_id: i32,
    episode_id: i32,
) -> Result<Option<(sonettobuf::LayerNo, i32)>> {
    let mut tx = pool.begin().await?;
    let result = sqlx::query(
        "UPDATE user_tower_episodes SET status = 0, assist_boss_id = 0
         WHERE user_id = ? AND tower_type = ? AND tower_id = ?
           AND layer_id = ? AND episode_id = ?",
    )
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .bind(layer_id)
    .bind(episode_id)
    .execute(&mut *tx)
    .await?;
    if result.rows_affected() != 1 {
        tx.rollback().await?;
        return Ok(None);
    }
    for table in [
        "user_tower_episode_hero_equips",
        "user_tower_episode_heroes",
    ] {
        sqlx::query(&format!(
            "DELETE FROM {table}
             WHERE user_id = ? AND tower_type = ? AND tower_id = ?
               AND layer_id = ? AND episode_id = ?"
        ))
        .bind(user_id)
        .bind(tower_type)
        .bind(tower_id)
        .bind(layer_id)
        .bind(episode_id)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;

    let layer = layer_info(pool, user_id, tower_type, tower_id, layer_id).await?;
    let history = tower_history_high_score(pool, user_id, tower_type, tower_id)
        .await?
        .unwrap_or_default();
    Ok(layer.map(|layer| (layer, history)))
}

pub async fn tower_history_high_score(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
) -> sqlx::Result<Option<i32>> {
    sqlx::query_scalar(
        "SELECT history_high_score FROM user_towers
         WHERE user_id = ? AND tower_type = ? AND tower_id = ?",
    )
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .fetch_optional(pool)
    .await
}

pub async fn layer_info(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
    layer_id: i32,
) -> Result<Option<sonettobuf::LayerNo>> {
    let score = sqlx::query_as::<_, (i32, i32)>(
        "SELECT curr_high_score, history_high_score FROM user_tower_layers
         WHERE user_id = ? AND tower_type = ? AND tower_id = ? AND layer_id = ?",
    )
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .bind(layer_id)
    .fetch_optional(pool)
    .await?;
    let Some((curr_high_score, history_high_score)) = score else {
        return Ok(None);
    };
    Ok(Some(sonettobuf::LayerNo {
        layer_id: Some(layer_id),
        curr_high_score: Some(curr_high_score),
        history_high_score: Some(history_high_score),
        episode_n_os: get_layer_episodes(pool, user_id, tower_type, tower_id, layer_id).await?,
    }))
}

pub async fn activate_talent(
    pool: &SqlitePool,
    user_id: i64,
    boss_id: i32,
    plan_id: i32,
    talent_id: i32,
    current_talent_point: i32,
    new_talent_point: i32,
) -> sqlx::Result<bool> {
    let mut tx = pool.begin().await?;
    let updated = sqlx::query(
        "UPDATE user_assist_boss_talent_plans SET talent_point = ?
         WHERE user_id = ? AND boss_id = ? AND plan_id = ? AND talent_point = ?",
    )
    .bind(new_talent_point)
    .bind(user_id)
    .bind(boss_id)
    .bind(plan_id)
    .bind(current_talent_point)
    .execute(&mut *tx)
    .await?;
    if updated.rows_affected() != 1 {
        return Ok(false);
    }
    let inserted = sqlx::query(
        "INSERT OR IGNORE INTO user_assist_boss_plan_talents
             (user_id, boss_id, plan_id, talent_id)
         VALUES (?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(boss_id)
    .bind(plan_id)
    .bind(talent_id)
    .execute(&mut *tx)
    .await?;
    if inserted.rows_affected() != 1 {
        return Ok(false);
    }
    tx.commit().await?;
    Ok(true)
}

pub async fn reset_talent(
    pool: &SqlitePool,
    user_id: i64,
    boss_id: i32,
    plan_id: i32,
    talent_id: Option<i32>,
    current_talent_point: i32,
    new_talent_point: i32,
) -> sqlx::Result<bool> {
    let mut tx = pool.begin().await?;
    let updated = sqlx::query(
        "UPDATE user_assist_boss_talent_plans SET talent_point = ?
         WHERE user_id = ? AND boss_id = ? AND plan_id = ? AND talent_point = ?",
    )
    .bind(new_talent_point)
    .bind(user_id)
    .bind(boss_id)
    .bind(plan_id)
    .bind(current_talent_point)
    .execute(&mut *tx)
    .await?;
    if updated.rows_affected() != 1 {
        return Ok(false);
    }
    let deleted = sqlx::query(
        "DELETE FROM user_assist_boss_plan_talents
         WHERE user_id = ? AND boss_id = ? AND plan_id = ? AND (? IS NULL OR talent_id = ?)",
    )
    .bind(user_id)
    .bind(boss_id)
    .bind(plan_id)
    .bind(talent_id)
    .bind(talent_id)
    .execute(&mut *tx)
    .await?;
    if talent_id.is_some() && deleted.rows_affected() != 1 {
        return Ok(false);
    }
    tx.commit().await?;
    Ok(true)
}

pub async fn change_talent_plan(
    pool: &SqlitePool,
    user_id: i64,
    boss_id: i32,
    plan_id: i32,
) -> sqlx::Result<bool> {
    let result = sqlx::query(
        "UPDATE user_assist_bosses SET use_talent_plan = ? WHERE user_id = ? AND boss_id = ?",
    )
    .bind(plan_id)
    .bind(user_id)
    .bind(boss_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn rename_active_talent_plan(
    pool: &SqlitePool,
    user_id: i64,
    boss_id: i32,
    plan_name: &str,
) -> sqlx::Result<bool> {
    let result = sqlx::query(
        "UPDATE user_assist_boss_talent_plans SET plan_name = ?
         WHERE user_id = ? AND boss_id = ? AND plan_id = (
             SELECT use_talent_plan FROM user_assist_bosses WHERE user_id = ? AND boss_id = ?
         )",
    )
    .bind(plan_name)
    .bind(user_id)
    .bind(boss_id)
    .bind(user_id)
    .bind(boss_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

/// Update tower layer score after battle
pub async fn update_tower_layer_score(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
    layer_id: i32,
    score: i32,
) -> sqlx::Result<()> {
    // Update current and history high scores
    sqlx::query(
        "UPDATE user_tower_layers
         SET curr_high_score = ?,
             history_high_score = MAX(history_high_score, ?)
         WHERE user_id = ? AND tower_type = ? AND tower_id = ? AND layer_id = ?",
    )
    .bind(score)
    .bind(score)
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .bind(layer_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Update tower episode status
pub async fn update_tower_episode_status(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
    layer_id: i32,
    episode_id: i32,
    status: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE user_tower_episodes
         SET status = ?
         WHERE user_id = ? AND tower_type = ? AND tower_id = ? AND layer_id = ? AND episode_id = ?",
    )
    .bind(status)
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .bind(layer_id)
    .bind(episode_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Update tower pass layer
pub async fn update_tower_pass_layer(
    pool: &SqlitePool,
    user_id: i64,
    tower_type: i32,
    tower_id: i32,
    pass_layer_id: i32,
    history_high_score: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE user_towers
         SET pass_layer_id = MAX(pass_layer_id, ?),
             history_high_score = MAX(history_high_score, ?)
         WHERE user_id = ? AND tower_type = ? AND tower_id = ?",
    )
    .bind(pass_layer_id)
    .bind(history_high_score)
    .bind(user_id)
    .bind(tower_type)
    .bind(tower_id)
    .execute(pool)
    .await?;

    Ok(())
}
