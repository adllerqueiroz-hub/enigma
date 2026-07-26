use super::*;

pub async fn act104_infos(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Get104InfosReply, AppError> {
    let tables = config::configs::get();
    let activity_id = activity_id
        .or_else(|| {
            tables
                .activity104_episode
                .iter()
                .map(|row| row.activity_id)
                .max()
        })
        .ok_or(AppError::InvalidRequest)?;
    let episode_states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act104Episode).await?;
    let special_states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act104Special).await?;
    let after_story_states = activity_state::get(
        db,
        player_id,
        activity_id,
        ActivityStateKind::Act104AfterStory,
    )
    .await?;
    let story_states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act104Story).await?;
    let summary_states = activity_state::get(
        db,
        player_id,
        activity_id,
        ActivityStateKind::Act104PopSummary,
    )
    .await?;

    let mut episodes = tables
        .activity104_episode
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .map(|row| Act104EpisodeNo {
            layer: Some(row.layer),
            state: Some(
                episode_states
                    .get(&row.layer)
                    .map(|(state, _, _)| *state)
                    .unwrap_or_default(),
            ),
            read_after_story: Some(
                after_story_states
                    .get(&row.layer)
                    .is_some_and(|(state, _, _)| *state != 0),
            ),
        })
        .collect::<Vec<_>>();
    episodes.sort_by_key(|episode| episode.layer.unwrap_or_default());

    let mut specials = tables
        .activity104_special
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .map(|row| Act104SpecialNo {
            layer: Some(row.layer),
            state: Some(
                special_states
                    .get(&row.layer)
                    .map(|(state, _, _)| *state)
                    .unwrap_or_default(),
            ),
        })
        .collect::<Vec<_>>();
    specials.sort_by_key(|special| special.layer.unwrap_or_default());

    let mut retails = tables
        .activity104_retail
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .map(|row| Act104RetailNo {
            id: Some(first_number(&row.retail_episode_id_pool).unwrap_or(row.stage)),
            state: Some(0),
            advanced_id: Some(0),
            star: Some(0),
            show_activity104_equip_ids: Vec::new(),
            position: Some(row.stage),
            advanced_rare: Some(0),
            tag: Some(0),
        })
        .collect::<Vec<_>>();
    retails.sort_by_key(|retail| retail.id.unwrap_or_default());

    Ok(Get104InfosReply {
        activity_id: Some(activity_id),
        activity104_items: Vec::new(),
        episodes,
        retails,
        specials,
        unlock_equip_indexs: Vec::new(),
        optional_equip_count: Some(0),
        hero_group_snapshot: Vec::new(),
        hero_group_snapshot_sub_id: Some(1),
        retail_stage: Some(1),
        read_activity104_story: Some(activity_flag(&story_states)),
        unlock_activity104_equip_ids: Vec::new(),
        trial: Some(Act104TrialNo {
            id: Some(
                tables
                    .activity104_trial
                    .iter()
                    .find(|row| row.activity_id == activity_id)
                    .map(|row| row.layer)
                    .unwrap_or_default(),
            ),
        }),
        pre_summary: Some(Act104PreSummaryNo {
            is_pop_summary: Some(activity_flag(&summary_states)),
            max_layer: Some(
                episode_states
                    .iter()
                    .filter_map(|(layer, (state, _, _))| (*state == 1).then_some(*layer))
                    .max()
                    .unwrap_or_default(),
            ),
        }),
    })
}

pub async fn mark_activity104_story(
    db: &SqlitePool,
    player_id: i64,
    activity_id: i32,
) -> Result<MarkActivity104StoryReply, AppError> {
    mark_activity104_flag(db, player_id, activity_id, ActivityStateKind::Act104Story).await?;
    Ok(MarkActivity104StoryReply {
        activity_id: Some(activity_id),
    })
}

pub async fn mark_pop_summary(
    db: &SqlitePool,
    player_id: i64,
    activity_id: i32,
) -> Result<MarkPopSummaryReply, AppError> {
    mark_activity104_flag(
        db,
        player_id,
        activity_id,
        ActivityStateKind::Act104PopSummary,
    )
    .await?;
    Ok(MarkPopSummaryReply {
        activity_id: Some(activity_id),
    })
}

async fn mark_activity104_flag(
    db: &SqlitePool,
    player_id: i64,
    activity_id: i32,
    kind: ActivityStateKind,
) -> Result<(), AppError> {
    if !config::configs::get()
        .activity104_episode
        .iter()
        .any(|row| row.activity_id == activity_id)
    {
        return Err(AppError::InvalidRequest);
    }
    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind,
            entry_id: 0,
            state: 1,
            progress: 0,
            ext: "",
        },
    )
    .await?;
    Ok(())
}

fn activity_flag(states: &activity_state::ActivityStates) -> bool {
    states.get(&0).is_some_and(|(state, _, _)| *state != 0)
}

pub async fn mark_episode_after_story(
    db: &SqlitePool,
    player_id: i64,
    activity_id: i32,
    layer: i32,
) -> Result<MarkEpisodeAfterStoryReply, AppError> {
    if !config::configs::get()
        .activity104_episode
        .iter()
        .any(|row| row.activity_id == activity_id && row.layer == layer)
    {
        return Err(AppError::InvalidRequest);
    }

    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act104AfterStory,
            entry_id: layer,
            state: 1,
            progress: 0,
            ext: "",
        },
    )
    .await?;

    Ok(MarkEpisodeAfterStoryReply {
        activity_id: Some(activity_id),
        layer: Some(layer),
    })
}

fn first_number(value: &str) -> Option<i32> {
    value
        .split('#')
        .find(|part| !part.is_empty())
        .and_then(|part| part.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_retail_episode_pool() {
        assert_eq!(first_number("1102101#1102102#1102103"), Some(1_102_101));
        assert_eq!(first_number(""), None);
    }

    #[tokio::test]
    async fn activity104_markers_round_trip_through_distinct_states() {
        let data_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("data/excel2json");
        let _ = config::init(data_dir.to_str().unwrap());
        let episode = config::configs::get()
            .activity104_episode
            .iter()
            .next()
            .unwrap();
        let (activity_id, layer) = (episode.activity_id, episode.layer);
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        database::run_migrations(&pool).await.unwrap();
        sqlx::query(
            "INSERT INTO users (id, username, created_at, updated_at) VALUES (7, 'act104', 0, 0)",
        )
        .execute(&pool)
        .await
        .unwrap();

        mark_episode_after_story(&pool, 7, activity_id, layer)
            .await
            .unwrap();
        mark_activity104_story(&pool, 7, activity_id).await.unwrap();
        mark_pop_summary(&pool, 7, activity_id).await.unwrap();
        let info = act104_infos(&pool, 7, Some(activity_id)).await.unwrap();

        assert_eq!(
            info.episodes
                .iter()
                .find(|episode| episode.layer == Some(layer))
                .and_then(|episode| episode.read_after_story),
            Some(true)
        );
        assert_eq!(info.read_activity104_story, Some(true));
        assert_eq!(
            info.pre_summary.and_then(|summary| summary.is_pop_summary),
            Some(true)
        );
    }
}
