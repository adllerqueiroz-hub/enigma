use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct NecrologistStoryState {
    pub user_id: i64,
    pub story_id: i32,
    pub info: String,
    pub updated_at: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct NecrologistStoryPlotState {
    pub user_id: i64,
    pub story_id: i32,
    pub plot_id: i32,
    pub state: i32,
    pub values_json: String,
    pub selected_options_json: String,
    pub unlock_end_ids_json: String,
    pub last_selected_options_json: String,
    pub last_end_id: i32,
    pub updated_at: i64,
}
