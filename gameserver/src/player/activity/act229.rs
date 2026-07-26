use crate::error::AppError;
use serde::Deserialize;
use sonettobuf::{Act229HeroNo, Act229StageNo, GetAct229InfoReply};
use sqlx::SqlitePool;

#[derive(Deserialize)]
struct SavedHero {
    hero_id: Option<i32>,
    equip_uids: Vec<i64>,
}

pub async fn act229_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<GetAct229InfoReply, AppError> {
    let tables = config::configs::get();
    let activity_id = activity_id
        .or_else(|| {
            tables
                .activity229_const
                .get(2)
                .and_then(|row| row.value.parse().ok())
        })
        .ok_or(AppError::InvalidRequest)?;
    let saved = sqlx::query_as::<_, (i32, i32, i32, i32, i32, String)>(
        "SELECT stage_id, star, max_star, round, min_round, heroes_json
         FROM user_activity229_stages
         WHERE user_id = ? AND activity_id = ?",
    )
    .bind(player_id)
    .bind(activity_id)
    .fetch_all(db)
    .await?;

    let mut stages = tables
        .activity229_episode
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .map(|row| {
            let state = saved.iter().find(|state| state.0 == row.stage);
            let heroes = state.map(|state| heroes(&state.5)).unwrap_or_default();

            Act229StageNo {
                stage_id: Some(row.stage),
                star: Some(state.map(|state| state.1).unwrap_or_default()),
                max_star: Some(state.map(|state| state.2).unwrap_or_default()),
                round: Some(state.map(|state| state.3).unwrap_or_default()),
                min_round: Some(state.map(|state| state.4).unwrap_or_default()),
                heros: heroes,
            }
        })
        .collect::<Vec<_>>();
    stages.sort_by_key(|stage| stage.stage_id.unwrap_or_default());

    Ok(GetAct229InfoReply {
        activity_id: Some(activity_id),
        stages,
    })
}

fn heroes(json: &str) -> Vec<Act229HeroNo> {
    serde_json::from_str::<Vec<SavedHero>>(json)
        .unwrap_or_default()
        .into_iter()
        .map(|hero| Act229HeroNo {
            hero_id: hero.hero_id,
            equip_uids: hero.equip_uids,
        })
        .collect()
}
