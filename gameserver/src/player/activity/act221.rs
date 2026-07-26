use super::*;
use rand::Rng;
use sonettobuf::{
    Act221HeroInfo, Act221Info, Act221SelectReply, Act221SummonReply, Get221InfoReply,
};

const STATE_ENTRY_ID: i32 = 0;
const SUMMON_COUNT: usize = 10;

#[derive(Default, Deserialize, Serialize)]
struct Act221State {
    saved_hero_ids: Vec<Vec<i32>>,
    select_index: i32,
}

pub struct Act221Claim {
    pub reply: Act221SelectReply,
    pub rewards: reward::AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn act221_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Get221InfoReply, AppError> {
    Ok(Get221InfoReply {
        info: Some(info(db, player_id, activity_id).await?),
    })
}

pub async fn act221_summon(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Act221SummonReply, AppError> {
    let cfg = activity_cfg(activity_id)?;
    let mut state = load_state(db, player_id, cfg.activity_id).await?;
    if state.select_index != 0 || left_times(cfg, &state) <= 0 {
        return Err(AppError::InvalidRequest);
    }

    let heroes = summon_heroes(cfg)?;
    state.saved_hero_ids.push(heroes);
    save_state(db, player_id, cfg.activity_id, &state).await?;

    Ok(Act221SummonReply {
        info: Some(to_info(cfg.activity_id, left_times(cfg, &state), state)),
    })
}

pub async fn act221_select(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    select_index: Option<i32>,
) -> Result<Act221Claim, AppError> {
    let cfg = activity_cfg(activity_id)?;
    let mut state = load_state(db, player_id, cfg.activity_id).await?;
    if state.select_index != 0 {
        return Err(AppError::InvalidRequest);
    }

    let select_index = select_index.ok_or(AppError::InvalidRequest)?;
    let heroes = state
        .saved_hero_ids
        .get((select_index - 1).max(0) as usize)
        .ok_or(AppError::InvalidRequest)?
        .clone();

    let reward_set = reward::RewardSet {
        heroes: heroes.into_iter().map(|hero_id| (hero_id, 1)).collect(),
        ..Default::default()
    };
    let material_changes = reward_set.material_changes();
    let rewards = reward::apply(db, player_id, reward_set).await?;

    state.select_index = select_index;
    save_state(db, player_id, cfg.activity_id, &state).await?;

    Ok(Act221Claim {
        reply: Act221SelectReply {
            info: Some(to_info(cfg.activity_id, left_times(cfg, &state), state)),
        },
        rewards,
        material_changes,
    })
}

async fn info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Act221Info, AppError> {
    let cfg = activity_cfg(activity_id)?;
    let state = load_state(db, player_id, cfg.activity_id).await?;
    Ok(to_info(cfg.activity_id, left_times(cfg, &state), state))
}

fn activity_cfg(
    activity_id: Option<i32>,
) -> Result<&'static config::activity221::Activity221, AppError> {
    let tables = config::configs::get();
    activity_id
        .and_then(|activity_id| {
            tables
                .activity221
                .iter()
                .find(|row| row.activity_id == activity_id)
        })
        .or_else(|| tables.activity221.iter().max_by_key(|row| row.activity_id))
        .ok_or(AppError::InvalidRequest)
}

fn to_info(activity_id: i32, left_times: i32, state: Act221State) -> Act221Info {
    Act221Info {
        activity_id: Some(activity_id),
        left_times: Some(left_times),
        saved_hero_ids: state
            .saved_hero_ids
            .into_iter()
            .map(|hero_id| Act221HeroInfo { hero_id })
            .collect(),
        select_index: Some(state.select_index),
    }
}

async fn load_state(
    db: &SqlitePool,
    player_id: i64,
    activity_id: i32,
) -> Result<Act221State, AppError> {
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act221Summon).await?;
    Ok(states
        .get(&STATE_ENTRY_ID)
        .and_then(|(_, _, ext)| serde_json::from_str(ext).ok())
        .unwrap_or_default())
}

async fn save_state(
    db: &SqlitePool,
    player_id: i64,
    activity_id: i32,
    state: &Act221State,
) -> Result<(), AppError> {
    let ext = serde_json::to_string(state)?;
    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act221Summon,
            entry_id: STATE_ENTRY_ID,
            state: state.select_index,
            progress: state.saved_hero_ids.len() as i32,
            ext: &ext,
        },
    )
    .await?;
    Ok(())
}

fn left_times(cfg: &config::activity221::Activity221, state: &Act221State) -> i32 {
    (cfg.summon_times - state.saved_hero_ids.len() as i32).max(0)
}

fn summon_heroes(cfg: &config::activity221::Activity221) -> Result<Vec<i32>, AppError> {
    let pool = crate::logic::summon::build_gacha_pool(cfg.pool_id, None)?;
    let weights = parse_weights(&cfg.init_weight);
    if weights.is_empty() {
        return Err(AppError::InvalidRequest);
    }

    let mut rng = rand::rng();
    let extra_top_index =
        (rng.random_range(0..100) < cfg.double_six_rate).then(|| rng.random_range(1..SUMMON_COUNT));
    let mut heroes = vec![pool.choose_config_rarity(5, &mut rng)?];
    while heroes.len() < SUMMON_COUNT {
        let rarity = if extra_top_index == Some(heroes.len()) {
            5
        } else {
            choose_weighted(&weights, &mut rng)?
        };
        heroes.push(pool.choose_config_rarity(rarity, &mut rng)?);
    }

    Ok(heroes)
}

fn parse_weights(value: &str) -> Vec<(i32, u32)> {
    value
        .split('|')
        .filter_map(|part| {
            let mut fields = part.split('#');
            Some((fields.next()?.parse().ok()?, fields.next()?.parse().ok()?))
        })
        .collect()
}

fn choose_weighted(weights: &[(i32, u32)], rng: &mut impl Rng) -> Result<i32, AppError> {
    let total = weights.iter().map(|(_, weight)| *weight).sum::<u32>();
    if total == 0 {
        return Err(AppError::InvalidRequest);
    }

    let mut roll = rng.random_range(0..total);
    for (rarity, weight) in weights {
        if roll < *weight {
            return Ok(*rarity);
        }
        roll -= *weight;
    }

    Ok(weights[0].0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn activity221_config_loads_simulation_pool() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);
        let row = config::configs::get()
            .activity221
            .iter()
            .next()
            .expect("activity221 config exists");

        assert!(row.pool_id > 0);
        assert!(row.summon_times > 0);
        assert!(
            config::configs::get()
                .summon
                .iter()
                .any(|summon| summon.id == row.pool_id)
        );
    }
}
