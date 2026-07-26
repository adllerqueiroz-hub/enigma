use sonettobuf::{Task, TaskActivityInfo};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UserTask {
    pub user_id: i64,
    pub type_id: i32,
    pub task_id: i32,
    pub progress: i32,
    pub has_finished: bool,
    pub finish_count: i32,
    pub expiry_time: i32,
    pub min_type_id: i32,
    pub activity_id: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<UserTask> for Task {
    fn from(task: UserTask) -> Self {
        Task {
            id: task.task_id,
            progress: task.progress,
            has_finished: task.has_finished,
            finish_count: Some(task.finish_count),
            r#type: Some(task.type_id),
            expiry_time: Some(task.expiry_time),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct UserTaskActivity {
    pub user_id: i64,
    pub type_id: i32,
    pub define_id: i32,
    pub value: i32,
    pub gain_value: i32,
    pub expiry_time: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<UserTaskActivity> for TaskActivityInfo {
    fn from(activity: UserTaskActivity) -> Self {
        TaskActivityInfo {
            type_id: activity.type_id,
            define_id: activity.define_id,
            value: activity.value,
            gain_value: Some(activity.gain_value),
            expiry_time: activity.expiry_time,
        }
    }
}
