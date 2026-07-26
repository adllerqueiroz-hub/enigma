use super::*;

const ACT146_UNFINISHED: i32 = 0;
const ACT146_FINISHED: i32 = 1;
const ACT146_RECEIVED: i32 = 2;

pub struct Act146Claim {
    pub reply: Act146EpisodeBonusReply,
    pub rewards: reward::AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn act146_infos(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<GetAct146InfosReply, AppError> {
    let activity_id = act146_activity_id(activity_id)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act146Episode).await?;
    let mut episodes = config::configs::get()
        .activity146
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .map(|row| Act146Episode {
            id: Some(row.id),
            state: Some(
                states
                    .get(&row.id)
                    .map(|(state, _, _)| *state)
                    .unwrap_or(ACT146_UNFINISHED),
            ),
        })
        .collect::<Vec<_>>();
    episodes.sort_by_key(|episode| episode.id.unwrap_or_default());

    Ok(GetAct146InfosReply {
        activity_id: Some(activity_id),
        act146_episodes: episodes,
    })
}

pub async fn finish_act146_episode(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    episode_id: Option<i32>,
) -> Result<FinishAct146EpisodeReply, AppError> {
    let activity_id = act146_activity_id(activity_id)?;
    let episode_id = valid_act146_episode(activity_id, episode_id)?;
    set_act146_episode_state(db, player_id, activity_id, episode_id, ACT146_FINISHED).await?;

    Ok(FinishAct146EpisodeReply {
        activity_id: Some(activity_id),
        episode_id: Some(episode_id),
        update_act146_episodes: vec![Act146Episode {
            id: Some(episode_id),
            state: Some(ACT146_FINISHED),
        }],
    })
}

pub async fn act146_episode_bonus(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    episode_id: Option<i32>,
) -> Result<Act146Claim, AppError> {
    let activity_id = act146_activity_id(activity_id)?;
    let episode_id = valid_act146_episode(activity_id, episode_id)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act146Episode).await?;

    if states
        .get(&episode_id)
        .map(|(state, _, _)| *state)
        .unwrap_or(ACT146_UNFINISHED)
        != ACT146_FINISHED
    {
        return Err(AppError::InvalidRequest);
    }

    let bonus = config::configs::get()
        .activity146
        .iter()
        .find(|row| row.activity_id == activity_id && row.id == episode_id)
        .ok_or(AppError::InvalidRequest)?
        .bonus
        .clone();
    set_act146_episode_state(db, player_id, activity_id, episode_id, ACT146_RECEIVED).await?;

    let parsed = reward::parse(&bonus);
    let material_changes = parsed.material_changes();
    let rewards = reward::apply(db, player_id, parsed).await?;

    Ok(Act146Claim {
        reply: Act146EpisodeBonusReply {
            activity_id: Some(activity_id),
            episode_id: Some(episode_id),
            update_act146_episodes: vec![Act146Episode {
                id: Some(episode_id),
                state: Some(ACT146_RECEIVED),
            }],
        },
        rewards,
        material_changes,
    })
}

fn act146_activity_id(activity_id: Option<i32>) -> Result<i32, AppError> {
    activity_id
        .or_else(|| {
            config::configs::get()
                .activity146
                .iter()
                .next()
                .map(|row| row.activity_id)
        })
        .ok_or(AppError::InvalidRequest)
}

fn valid_act146_episode(activity_id: i32, episode_id: Option<i32>) -> Result<i32, AppError> {
    let episode_id = episode_id.ok_or(AppError::InvalidRequest)?;
    config::configs::get()
        .activity146
        .iter()
        .any(|row| row.activity_id == activity_id && row.id == episode_id)
        .then_some(episode_id)
        .ok_or(AppError::InvalidRequest)
}

async fn set_act146_episode_state(
    db: &SqlitePool,
    player_id: i64,
    activity_id: i32,
    episode_id: i32,
    state: i32,
) -> Result<(), AppError> {
    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act146Episode,
            entry_id: episode_id,
            state,
            progress: 0,
            ext: "",
        },
    )
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn act146_default_activity_id_comes_from_config() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);

        assert_eq!(act146_activity_id(None).unwrap(), 11511);
    }
}
