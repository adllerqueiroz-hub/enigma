// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushBoxTask {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    pub desc: String,
    #[serde(rename = "isOnline")]
    pub is_online: i32,
    #[serde(rename = "listenerParam")]
    pub listener_param: String,
    #[serde(rename = "listenerType")]
    pub listener_type: String,
    #[serde(rename = "maxProgress")]
    pub max_progress: i32,
    #[serde(rename = "minTypeId")]
    pub min_type_id: i32,
    pub sort: i32,
    #[serde(rename = "taskId")]
    pub task_id: i32,
}
pub struct PushBoxTaskTable {
    records: Vec<PushBoxTask>,
}

impl PushBoxTaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<PushBoxTask> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[PushBoxTask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, PushBoxTask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}