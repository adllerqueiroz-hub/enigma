use super::*;
use sonettobuf::{
    Act216TaskInfo, FinishAct216TaskReply, GetAct216InfoReply, GetAct216OnceBonusReply,
};
use std::collections::HashSet;

enum Act216Flag {
    OnceBonus,
    TalentItem,
}

impl Act216Flag {
    const fn id(self) -> i32 {
        match self {
            Self::OnceBonus => 1,
            Self::TalentItem => 2,
        }
    }
}

pub struct Act216TaskClaim {
    pub reply: FinishAct216TaskReply,
    pub task_info: Act216TaskInfo,
    pub rewards: Option<reward::AppliedRewards>,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub struct Act216OnceBonusClaim {
    pub reply: GetAct216OnceBonusReply,
    pub rewards: Option<reward::AppliedRewards>,
    pub material_changes: Vec<(u32, u32, i32)>,
}

struct HeroSnapshot {
    hero_id: i32,
    level: i32,
    rank: i32,
    talent: i32,
}

pub async fn act216_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<GetAct216InfoReply, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act216Task).await?;
    let flags =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act216Flag).await?;
    let heroes = load_heroes(db, player_id).await?;

    Ok(GetAct216InfoReply {
        activity_id: Some(activity_id),
        get_once_bonus: Some(flag(&flags, Act216Flag::OnceBonus)),
        task_infos: task_infos(activity_id, &states, &heroes),
        has_use_talent_item: Some(flag(&flags, Act216Flag::TalentItem)),
    })
}

pub async fn finish_act216_task(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    task_id: Option<i32>,
) -> Result<Act216TaskClaim, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let task_id = task_id.ok_or(AppError::InvalidRequest)?;
    let row = task_row(activity_id, task_id).ok_or(AppError::InvalidRequest)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act216Task).await?;
    let heroes = load_heroes(db, player_id).await?;
    let task_info = task_info(row, &states, &heroes);

    if task_info.has_finish.unwrap_or_default() {
        return Ok(Act216TaskClaim {
            reply: FinishAct216TaskReply {
                activity_id: Some(activity_id),
                task_id: Some(task_id),
            },
            task_info,
            rewards: None,
            material_changes: Vec::new(),
        });
    }
    if task_info.progress.unwrap_or_default() < row.max_progress {
        return Err(AppError::InvalidRequest);
    }

    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act216Task,
            entry_id: task_id,
            state: 1,
            progress: row.max_progress,
            ext: "",
        },
    )
    .await?;

    let parsed = reward::parse(&row.bonus);
    let material_changes = parsed.material_changes();
    let rewards = reward::apply(db, player_id, parsed).await?;

    Ok(Act216TaskClaim {
        reply: FinishAct216TaskReply {
            activity_id: Some(activity_id),
            task_id: Some(task_id),
        },
        task_info: Act216TaskInfo {
            task_id: Some(task_id),
            progress: Some(row.max_progress),
            has_finish: Some(true),
        },
        rewards: Some(rewards),
        material_changes,
    })
}

pub async fn get_act216_once_bonus(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Act216OnceBonusClaim, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let row = config::configs::get()
        .activity216_once_bonus
        .iter()
        .find(|row| row.activity_id == activity_id)
        .ok_or(AppError::InvalidRequest)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act216Task).await?;
    let flags =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act216Flag).await?;
    let heroes = load_heroes(db, player_id).await?;

    if flag(&flags, Act216Flag::OnceBonus) {
        return Ok(Act216OnceBonusClaim {
            reply: GetAct216OnceBonusReply {
                activity_id: Some(activity_id),
            },
            rewards: None,
            material_changes: Vec::new(),
        });
    }

    let finished = task_infos(activity_id, &states, &heroes)
        .into_iter()
        .filter(|task| task.has_finish.unwrap_or_default())
        .count() as i32;
    if finished < row.need_finish_task_num {
        return Err(AppError::InvalidRequest);
    }

    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act216Flag,
            entry_id: Act216Flag::OnceBonus.id(),
            state: 1,
            progress: 0,
            ext: "",
        },
    )
    .await?;

    let parsed = reward::parse(&row.bonus);
    let material_changes = parsed.material_changes();
    let rewards = reward::apply(db, player_id, parsed).await?;

    Ok(Act216OnceBonusClaim {
        reply: GetAct216OnceBonusReply {
            activity_id: Some(activity_id),
        },
        rewards: Some(rewards),
        material_changes,
    })
}

fn resolve_activity_id(activity_id: Option<i32>) -> Result<i32, AppError> {
    activity_id
        .or_else(|| {
            config::configs::get()
                .activity216_task
                .iter()
                .map(|row| row.activity_id)
                .max()
        })
        .ok_or(AppError::InvalidRequest)
}

fn task_row(
    activity_id: i32,
    task_id: i32,
) -> Option<&'static config::activity216_task::Activity216Task> {
    config::configs::get()
        .activity216_task
        .get(task_id)
        .filter(|row| row.activity_id == activity_id && row.is_online != 0)
}

fn task_infos(
    activity_id: i32,
    states: &activity_state::ActivityStates,
    heroes: &[HeroSnapshot],
) -> Vec<Act216TaskInfo> {
    let mut infos = config::configs::get()
        .activity216_task
        .iter()
        .filter(|row| row.activity_id == activity_id && row.is_online != 0)
        .map(|row| task_info(row, states, heroes))
        .collect::<Vec<_>>();
    infos.sort_by_key(|info| info.task_id.unwrap_or_default());
    infos
}

fn task_info(
    row: &config::activity216_task::Activity216Task,
    states: &activity_state::ActivityStates,
    heroes: &[HeroSnapshot],
) -> Act216TaskInfo {
    let (state, stored_progress, _) = states
        .get(&row.id)
        .cloned()
        .unwrap_or((0, 0, String::new()));
    let has_finish = state != 0;
    let progress = if has_finish {
        row.max_progress
    } else {
        stored_progress.max(dynamic_progress(row, heroes))
    };

    Act216TaskInfo {
        task_id: Some(row.id),
        progress: Some(progress.min(row.max_progress)),
        has_finish: Some(has_finish),
    }
}

fn dynamic_progress(
    row: &config::activity216_task::Activity216Task,
    heroes: &[HeroSnapshot],
) -> i32 {
    match row.listener_type.as_str() {
        "UpdateTargetRareHeroRank" => rare_rank_progress(&row.listener_param, heroes),
        "UpdateTalent" => talent_progress(&row.listener_param, heroes),
        "UpdateTargetRareHeroLevel" => rare_level_progress(&row.listener_param, heroes),
        _ => 0,
    }
}

fn rare_rank_progress(param: &str, heroes: &[HeroSnapshot]) -> i32 {
    let parts = parse_i32s(param, '#');
    let [rare, rank] = parts.as_slice() else {
        return 0;
    };
    heroes
        .iter()
        .filter(|hero| hero_rare(hero.hero_id) == Some(*rare) && hero.rank >= *rank)
        .count() as i32
}

fn talent_progress(param: &str, heroes: &[HeroSnapshot]) -> i32 {
    let parts = parse_i32s(param, '#');
    let [rare, talent] = parts.as_slice() else {
        return 0;
    };
    heroes
        .iter()
        .filter(|hero| hero_rare(hero.hero_id) == Some(*rare) && hero.talent >= *talent)
        .count() as i32
}

fn rare_level_progress(param: &str, heroes: &[HeroSnapshot]) -> i32 {
    rare_level_progress_with(param, heroes, hero_rare)
}

fn rare_level_progress_with(
    param: &str,
    heroes: &[HeroSnapshot],
    rare_of: impl Fn(i32) -> Option<i32>,
) -> i32 {
    let parts = param.split('|').collect::<Vec<_>>();
    let Some(rare) = parts.first().and_then(|part| part.parse::<i32>().ok()) else {
        return 0;
    };
    let Some(level) = parts.get(1).and_then(|part| part.parse::<i32>().ok()) else {
        return 0;
    };
    let ids = parts
        .get(2)
        .map(|part| parse_i32s(part, '#').into_iter().collect::<HashSet<_>>());

    heroes
        .iter()
        .filter(|hero| {
            rare_of(hero.hero_id) == Some(rare)
                && hero.level >= level
                && ids.as_ref().is_none_or(|ids| ids.contains(&hero.hero_id))
        })
        .count() as i32
}

fn hero_rare(hero_id: i32) -> Option<i32> {
    config::configs::get()
        .character
        .get(hero_id)
        .map(|hero| hero.rare)
}

fn parse_i32s(value: &str, separator: char) -> Vec<i32> {
    value
        .split(separator)
        .filter_map(|part| part.parse::<i32>().ok())
        .collect()
}

fn flag(states: &activity_state::ActivityStates, flag: Act216Flag) -> bool {
    states
        .get(&flag.id())
        .is_some_and(|(state, _, _)| *state != 0)
}

async fn load_heroes(db: &SqlitePool, player_id: i64) -> Result<Vec<HeroSnapshot>, AppError> {
    let rows = sqlx::query_as::<_, (i32, i32, i32, i32)>(
        "SELECT hero_id, level, rank, talent FROM heroes WHERE user_id = ?",
    )
    .bind(player_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(hero_id, level, rank, talent)| HeroSnapshot {
            hero_id,
            level,
            rank,
            talent,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_act216_rare_level_with_optional_hero_ids() {
        let heroes = vec![HeroSnapshot {
            hero_id: 3125,
            level: 180,
            rank: 0,
            talent: 0,
        }];

        let rare_of = |_| Some(5);
        assert_eq!(
            rare_level_progress_with("5|180|3120#3126#3125", &heroes, rare_of),
            1
        );
        assert_eq!(rare_level_progress_with("5|180|3120", &heroes, rare_of), 0);
    }
}
