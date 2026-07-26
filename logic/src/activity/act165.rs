use super::*;

pub struct Act165RewardClaim {
    pub reply: Act165GainMilestoneRewardReply,
    pub rewards: reward::AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct Act165State {
    infer_state: i32,
    gained_ending_count: i32,
    steps: Vec<Act165StepState>,
    endings: Vec<Act165EndingState>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Act165StepState {
    step_id: i32,
    keywords: Vec<i32>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Act165EndingState {
    ending_id: i32,
    steps: Vec<Act165StepState>,
}

pub async fn act165_get_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Act165GetInfoReply, AppError> {
    let activity_id = activity_id.unwrap_or_else(latest_act165_activity_id);
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act165Story).await?;
    let mut stories = config::configs::get()
        .activity165_story
        .iter()
        .filter(|row| row.activity_id == activity_id)
        .collect::<Vec<_>>();
    stories.sort_by_key(|row| std::cmp::Reverse(row.story_id));

    Ok(Act165GetInfoReply {
        activity_id: Some(activity_id),
        story_infos: stories
            .into_iter()
            .map(|row| act165_story_info(row, states.get(&row.story_id)))
            .collect(),
    })
}

pub async fn act165_modify_keyword(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    story_id: Option<i32>,
    keyword_ids: Vec<i32>,
) -> Result<Act165ModifyKeywordReply, AppError> {
    let activity_id = activity_id.unwrap_or_else(latest_act165_activity_id);
    let story_id = story_id.ok_or(AppError::InvalidRequest)?;
    let story = act165_story(activity_id, story_id)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act165Story).await?;
    let (story_state, _, ext) = states.get(&story_id).cloned().unwrap_or((
        i32::from(story.pre_element_id1 == 0),
        story.first_step_id,
        String::new(),
    ));
    if story_state == 0 {
        return Err(AppError::InvalidRequest);
    }

    let mut state = act165_state(&ext, story.first_step_id);
    let index = state.steps.len().saturating_sub(1);
    let step_id = state
        .steps
        .get(index)
        .map(|step| step.step_id)
        .unwrap_or(story.first_step_id);
    act165_validate_keywords(story_id, step_id, &keyword_ids)?;
    state.steps[index].keywords = keyword_ids.clone();

    if let Some(next_step_id) = act165_next_step(story_id, step_id, &keyword_ids) {
        act165_unlock_next_steps(story_id, &mut state, next_step_id);
    }
    if state
        .steps
        .last()
        .is_some_and(|step| act165_is_ending_step(story_id, step.step_id))
    {
        state.infer_state = 1;
    }

    save_act165_state(db, player_id, activity_id, story_id, story_state, &state).await?;
    let ext = serde_json::to_string(&state)?;
    let progress = state.steps.last().map(|step| step.step_id).unwrap_or(0);

    Ok(Act165ModifyKeywordReply {
        activity_id: Some(activity_id),
        curr_keyword_ids: keyword_ids,
        story_info: Some(act165_story_info(
            story,
            Some(&(story_state, progress, ext)),
        )),
    })
}

pub async fn act165_generate_ending(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    story_id: Option<i32>,
) -> Result<Act165GenerateEndingReply, AppError> {
    let activity_id = activity_id.unwrap_or_else(latest_act165_activity_id);
    let story_id = story_id.ok_or(AppError::InvalidRequest)?;
    let story = act165_story(activity_id, story_id)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act165Story).await?;
    let (story_state, _, ext) = states.get(&story_id).cloned().unwrap_or((
        i32::from(story.pre_element_id1 == 0),
        story.first_step_id,
        String::new(),
    ));
    if story_state == 0 {
        return Err(AppError::InvalidRequest);
    }

    let mut state = act165_state(&ext, story.first_step_id);
    let final_step_id = state
        .steps
        .last()
        .map(|step| step.step_id)
        .ok_or(AppError::InvalidRequest)?;
    let ending = config::configs::get()
        .activity165_ending
        .iter()
        .find(|row| row.belong_story_id == story_id && row.final_step_id == final_step_id)
        .ok_or(AppError::InvalidRequest)?;
    let ending_info = EndingInfo {
        ending_id: Some(ending.ending_id),
        inferred_steps: state.steps.iter().map(act165_step_info).collect(),
    };
    if !state
        .endings
        .iter()
        .any(|saved| saved.ending_id == ending.ending_id)
    {
        state.endings.push(Act165EndingState {
            ending_id: ending.ending_id,
            steps: state.steps.clone(),
        });
    }
    state.infer_state = 2;
    save_act165_state(db, player_id, activity_id, story_id, story_state, &state).await?;

    Ok(Act165GenerateEndingReply {
        activity_id: Some(activity_id),
        story_id: Some(story_id),
        ending_info: Some(ending_info),
    })
}

pub async fn act165_restart(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    story_id: Option<i32>,
    step_id: Option<i32>,
) -> Result<Act165RestartReply, AppError> {
    let activity_id = activity_id.unwrap_or_else(latest_act165_activity_id);
    let story_id = story_id.ok_or(AppError::InvalidRequest)?;
    let story = act165_story(activity_id, story_id)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act165Story).await?;
    let (story_state, _, ext) = states.get(&story_id).cloned().unwrap_or((
        i32::from(story.pre_element_id1 == 0),
        story.first_step_id,
        String::new(),
    ));
    if story_state == 0 {
        return Err(AppError::InvalidRequest);
    }

    let mut state = act165_state(&ext, story.first_step_id);
    let keep_step_id = step_id.unwrap_or(story.first_step_id);
    let keep = state
        .steps
        .iter()
        .position(|step| step.step_id == keep_step_id)
        .map(|index| index + 1)
        .unwrap_or(1);
    state.steps.truncate(keep);
    for step in &mut state.steps {
        if step.step_id == keep_step_id {
            step.keywords.clear();
        }
    }
    state.infer_state = 0;
    save_act165_state(db, player_id, activity_id, story_id, story_state, &state).await?;
    let ext = serde_json::to_string(&state)?;
    let progress = state.steps.last().map(|step| step.step_id).unwrap_or(0);

    Ok(Act165RestartReply {
        activity_id: Some(activity_id),
        story_info: Some(act165_story_info(
            story,
            Some(&(story_state, progress, ext)),
        )),
    })
}

pub async fn act165_gain_milestone_reward(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    story_id: Option<i32>,
) -> Result<Act165RewardClaim, AppError> {
    let activity_id = activity_id.unwrap_or_else(latest_act165_activity_id);
    let story_id = story_id.ok_or(AppError::InvalidRequest)?;
    let story = act165_story(activity_id, story_id)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act165Story).await?;
    let (story_state, _, ext) = states.get(&story_id).cloned().unwrap_or((
        i32::from(story.pre_element_id1 == 0),
        story.first_step_id,
        String::new(),
    ));
    let mut state = act165_state(&ext, story.first_step_id);
    let unlock_count = state.endings.len() as i32;
    let next_count = state.gained_ending_count + 1;
    if next_count > unlock_count {
        return Err(AppError::InvalidRequest);
    }

    let reward_row = config::configs::get()
        .activity165_reward
        .iter()
        .filter(|row| row.story_id == story_id)
        .find(|row| row.ending_count == next_count)
        .ok_or(AppError::InvalidRequest)?;
    state.gained_ending_count = next_count;
    save_act165_state(db, player_id, activity_id, story_id, story_state, &state).await?;

    let parsed = reward::parse_reward_id(reward_row.bonus);
    let material_changes = parsed.material_changes();
    let rewards = reward::apply(db, player_id, parsed).await?;

    Ok(Act165RewardClaim {
        reply: Act165GainMilestoneRewardReply {
            activity_id: Some(activity_id),
            story_id: Some(story_id),
            gained_ending_count: Some(state.gained_ending_count),
        },
        rewards,
        material_changes,
    })
}

fn act165_story(
    activity_id: i32,
    story_id: i32,
) -> Result<&'static config::activity165_story::Activity165Story, AppError> {
    config::configs::get()
        .activity165_story
        .iter()
        .find(|row| row.activity_id == activity_id && row.story_id == story_id)
        .ok_or(AppError::InvalidRequest)
}

fn act165_state(ext: &str, first_step_id: i32) -> Act165State {
    let mut state = if ext.is_empty() {
        Act165State::default()
    } else {
        serde_json::from_str(ext).unwrap_or_default()
    };
    if state.steps.is_empty() {
        state.steps.push(Act165StepState {
            step_id: first_step_id,
            keywords: Vec::new(),
        });
    }

    state
}

fn act165_story_info(
    row: &config::activity165_story::Activity165Story,
    stored: Option<&(i32, i32, String)>,
) -> Act165StoryInfo {
    let (story_state, progress, ext) = stored.cloned().unwrap_or((
        i32::from(row.pre_element_id1 == 0),
        row.first_step_id,
        String::new(),
    ));
    let mut state = act165_state(&ext, row.first_step_id);
    if ext.is_empty() {
        state.steps[0].step_id = if progress == 0 {
            row.first_step_id
        } else {
            progress
        };
    }

    Act165StoryInfo {
        story_id: Some(row.story_id),
        story_state: Some(story_state),
        first_ele_cd_begin_time: Some(0),
        infer_state: Some(state.infer_state),
        inferred_steps: state.steps.iter().map(act165_step_info).collect(),
        unlock_ending_infos: state
            .endings
            .iter()
            .map(|ending| EndingInfo {
                ending_id: Some(ending.ending_id),
                inferred_steps: ending.steps.iter().map(act165_step_info).collect(),
            })
            .collect(),
        gained_ending_count: Some(state.gained_ending_count),
    }
}

fn act165_step_info(step: &Act165StepState) -> StepInfo {
    StepInfo {
        step_id: Some(step.step_id),
        step_keywords: step.keywords.clone(),
    }
}

fn act165_validate_keywords(
    story_id: i32,
    step_id: i32,
    keyword_ids: &[i32],
) -> Result<(), AppError> {
    let step = config::configs::get()
        .activity165_step
        .iter()
        .find(|row| row.belong_story_id == story_id && row.step_id == step_id)
        .ok_or(AppError::InvalidRequest)?;
    let allowed = parse_i32_list(&step.optional_keyword_ids, '#');
    if keyword_ids.iter().all(|id| allowed.contains(id)) {
        Ok(())
    } else {
        Err(AppError::InvalidRequest)
    }
}

fn act165_next_step(story_id: i32, step_id: i32, keyword_ids: &[i32]) -> Option<i32> {
    let step = config::configs::get()
        .activity165_step
        .iter()
        .find(|row| row.belong_story_id == story_id && row.step_id == step_id)?;
    if step.answers_keyword_ids == "-1" {
        return None;
    }

    parse_i32_rows(&step.answers_keyword_ids)
        .into_iter()
        .find(|row| !row.is_empty() && same_set(&row[1..], keyword_ids))
        .and_then(|row| row.first().copied())
}

fn act165_unlock_next_steps(story_id: i32, state: &mut Act165State, next_step_id: i32) {
    if !act165_can_append_step(story_id, state, next_step_id) {
        return;
    }
    if state.steps.iter().any(|step| step.step_id == next_step_id) {
        return;
    }
    state.steps.push(Act165StepState {
        step_id: next_step_id,
        keywords: Vec::new(),
    });

    loop {
        let Some(current_step_id) = state.steps.last().map(|step| step.step_id) else {
            break;
        };
        if act165_is_ending_step(story_id, current_step_id) {
            break;
        }
        let Some(next_step_id) = act165_fixed_next_step(story_id, state) else {
            break;
        };
        if state.steps.iter().any(|step| step.step_id == next_step_id) {
            break;
        }
        state.steps.push(Act165StepState {
            step_id: next_step_id,
            keywords: Vec::new(),
        });
    }
}

fn act165_fixed_next_step(story_id: i32, state: &Act165State) -> Option<i32> {
    let current_step_id = state.steps.last()?.step_id;
    for branch in act165_branches(story_id) {
        if !same_prefix(&branch, &act165_step_ids(state)) {
            continue;
        }
        let next_step_id = branch.get(state.steps.len()).copied()?;
        if act165_is_ending_step(story_id, next_step_id)
            || act165_answer_allows_empty(story_id, current_step_id, next_step_id)
        {
            return Some(next_step_id);
        }
    }

    None
}

fn act165_can_append_step(story_id: i32, state: &Act165State, next_step_id: i32) -> bool {
    let mut steps = act165_step_ids(state);
    steps.push(next_step_id);

    act165_branches(story_id)
        .iter()
        .any(|branch| same_prefix(branch, &steps))
}

fn act165_answer_allows_empty(story_id: i32, step_id: i32, next_step_id: i32) -> bool {
    config::configs::get()
        .activity165_step
        .iter()
        .find(|row| row.belong_story_id == story_id && row.step_id == step_id)
        .map(|step| {
            parse_i32_rows(&step.answers_keyword_ids)
                .into_iter()
                .any(|row| row.first() == Some(&next_step_id) && row.len() == 1)
        })
        .unwrap_or(false)
}

fn act165_is_ending_step(story_id: i32, step_id: i32) -> bool {
    config::configs::get().activity165_step.iter().any(|row| {
        row.belong_story_id == story_id && row.step_id == step_id && row.answers_keyword_ids == "-1"
    })
}

fn act165_branches(story_id: i32) -> Vec<Vec<i32>> {
    let ending_steps = config::configs::get()
        .activity165_step
        .iter()
        .filter(|row| row.belong_story_id == story_id && row.answers_keyword_ids == "-1")
        .map(|row| row.step_id)
        .collect::<HashSet<_>>();

    config::configs::get()
        .activity165_step
        .iter()
        .filter(|row| row.belong_story_id == story_id)
        .flat_map(|row| {
            parse_i32_rows(&row.next_step_condition_ids)
                .into_iter()
                .filter_map(|values| {
                    let next_step_id = values.first().copied()?;
                    if !ending_steps.contains(&next_step_id) {
                        return None;
                    }
                    let mut branch = values[1..].to_vec();
                    branch.push(row.step_id);
                    branch.push(next_step_id);
                    Some(branch)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn act165_step_ids(state: &Act165State) -> Vec<i32> {
    state.steps.iter().map(|step| step.step_id).collect()
}

fn parse_i32_rows(value: &str) -> Vec<Vec<i32>> {
    value
        .split('|')
        .filter(|part| !part.is_empty())
        .map(|part| parse_i32_list(part, '#'))
        .filter(|row| !row.is_empty())
        .collect()
}

fn parse_i32_list(value: &str, separator: char) -> Vec<i32> {
    value
        .split(separator)
        .filter_map(|part| part.parse::<i32>().ok())
        .collect()
}

fn same_set(left: &[i32], right: &[i32]) -> bool {
    left.len() == right.len() && left.iter().all(|value| right.contains(value))
}

fn same_prefix(branch: &[i32], steps: &[i32]) -> bool {
    branch.len() >= steps.len()
        && steps
            .iter()
            .enumerate()
            .all(|(index, step_id)| branch.get(index) == Some(step_id))
}

async fn save_act165_state(
    db: &SqlitePool,
    player_id: i64,
    activity_id: i32,
    story_id: i32,
    story_state: i32,
    state: &Act165State,
) -> Result<(), AppError> {
    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act165Story,
            entry_id: story_id,
            state: story_state,
            progress: state.steps.last().map(|step| step.step_id).unwrap_or(0),
            ext: &serde_json::to_string(state)?,
        },
    )
    .await?;

    Ok(())
}
