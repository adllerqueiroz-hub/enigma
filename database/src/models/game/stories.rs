use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct ProcessingStory {
    pub id: i64,
    pub user_id: i64,
    pub story_id: i32,
    pub step_id: i32,
    pub favor: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<ProcessingStory> for sonettobuf::ProcessingStoryInfo {
    fn from(s: ProcessingStory) -> Self {
        sonettobuf::ProcessingStoryInfo {
            story_id: Some(s.story_id),
            step_id: Some(s.step_id),
            favor: Some(s.favor),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct HeroStoryState {
    pub user_id: i64,
    pub story_id: i32,
    pub progress: i32,
    pub get_reward: bool,
    pub get_score_bonus: String,
    pub score: i32,
    pub challenge_wave: i32,
    pub challenge_max_wave: i32,
    pub get_challenge_reward: bool,
    pub unlock: bool,
    pub is_new: bool,
    pub updated_at: i64,
}

impl From<HeroStoryState> for sonettobuf::HeroStoryInfo {
    fn from(state: HeroStoryState) -> Self {
        sonettobuf::HeroStoryInfo {
            story_id: Some(state.story_id),
            progress: Some(state.progress),
            get_reward: Some(state.get_reward),
            get_score_bonus: serde_json::from_str(&state.get_score_bonus).unwrap_or_default(),
            score: Some(state.score),
            challenge_wave: Some(state.challenge_wave),
            challenge_max_wave: Some(state.challenge_max_wave),
            get_challenge_reward: Some(state.get_challenge_reward),
            unlock: Some(state.unlock),
            dispatch_infos: Vec::new(),
        }
    }
}
