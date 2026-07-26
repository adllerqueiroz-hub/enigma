use crate::error::AppError;
use database::db::game::{necrologist_story, stories};
use sonettobuf::{
    GetHeroStoryReply, GetNecrologistStoryReply, GetStoryFinishReply, GetStoryReply,
    NecrologistStory, NecrologistStoryPlotInfo, NecrologistStorySituationValue,
    UpdateNecrologistStoryReply, UpdateStoryReply,
};
use sqlx::SqlitePool;
use std::collections::BTreeMap;

pub struct StoryUpdate {
    pub reply: UpdateStoryReply,
    pub finished_story_id: Option<i32>,
}

pub async fn get_story(db: &SqlitePool, player_id: i64) -> Result<GetStoryReply, AppError> {
    Ok(GetStoryReply {
        finish_list: stories::get_finished_stories(db, player_id).await?,
        processing_list: stories::get_processing_stories(db, player_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    })
}

pub async fn get_story_finish(
    db: &SqlitePool,
    player_id: i64,
    story_id: i32,
) -> Result<GetStoryFinishReply, AppError> {
    Ok(GetStoryFinishReply {
        is_finish: Some(stories::is_story_finished(db, player_id, story_id).await?),
    })
}

pub async fn update_story(
    db: &SqlitePool,
    player_id: i64,
    story_id: i32,
    step_id: i32,
    favor: i32,
) -> Result<StoryUpdate, AppError> {
    if step_id == -1 {
        stories::finish_story(db, player_id, story_id).await?;
        return Ok(StoryUpdate {
            reply: UpdateStoryReply {},
            finished_story_id: Some(story_id),
        });
    }

    stories::update_processing_story(db, player_id, story_id, step_id, favor).await?;
    Ok(StoryUpdate {
        reply: UpdateStoryReply {},
        finished_story_id: None,
    })
}

pub async fn get_hero_story(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetHeroStoryReply, AppError> {
    let states = stories::get_hero_story_states(db, player_id).await?;

    Ok(GetHeroStoryReply {
        new_story_list: states
            .iter()
            .filter(|state| state.is_new)
            .map(|state| state.story_id)
            .collect(),
        story_infos: states.into_iter().map(Into::into).collect(),
        times: Vec::new(),
        left_num: Some(0),
        today_exchange: Some(0),
        week_progress: Some(0),
        week_has_get: Some(false),
    })
}

pub async fn get_necrologist_story(
    db: &SqlitePool,
    player_id: i64,
    story_id: i32,
    tables: &config::GameDB,
) -> Result<GetNecrologistStoryReply, AppError> {
    let states = necrologist_story::get_stories(db, player_id, story_id, tables).await?;
    let mut story = Vec::with_capacity(states.len());

    for state in states {
        let plot_infos = necrologist_story::get_plots(db, player_id, state.story_id)
            .await?
            .into_iter()
            .map(|plot| {
                let values = serde_json::from_str::<BTreeMap<String, i32>>(&plot.values_json)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(key, value)| NecrologistStorySituationValue {
                        key: Some(key),
                        value: Some(value),
                    })
                    .collect();

                NecrologistStoryPlotInfo {
                    id: Some(plot.plot_id),
                    state: Some(plot.state),
                    values,
                    selected_options: serde_json::from_str(&plot.selected_options_json)
                        .unwrap_or_default(),
                    unlock_end_ids: serde_json::from_str(&plot.unlock_end_ids_json)
                        .unwrap_or_default(),
                    last_selected_options: serde_json::from_str(&plot.last_selected_options_json)
                        .unwrap_or_default(),
                    last_end_id: Some(plot.last_end_id),
                }
            })
            .collect();

        story.push(NecrologistStory {
            story_id: Some(state.story_id),
            info: Some(state.info),
            plot_infos,
        });
    }

    Ok(GetNecrologistStoryReply { story })
}

pub async fn update_necrologist_story(
    db: &SqlitePool,
    player_id: i64,
    story_id: i32,
    info: String,
    plot_infos: Vec<NecrologistStoryPlotInfo>,
) -> Result<UpdateNecrologistStoryReply, AppError> {
    necrologist_story::update_story(db, player_id, story_id, info.clone(), plot_infos.clone())
        .await?;

    Ok(UpdateNecrologistStoryReply {
        story_id: Some(story_id),
        info: Some(info),
        plot_infos,
    })
}

#[cfg(test)]
mod test;
