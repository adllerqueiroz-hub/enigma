// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity114Task {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    pub desc: String,
    #[serde(rename = "listenerParam")]
    pub listener_param: String,
    #[serde(rename = "listenerType")]
    pub listener_type: String,
    #[serde(rename = "maxProgress")]
    pub max_progress: i32,
    #[serde(rename = "minTypeId")]
    pub min_type_id: i32,
    #[serde(rename = "taskId")]
    pub task_id: i32,
}
pub struct Activity114TaskTable {
    records: Vec<Activity114Task>,
}

impl Activity114TaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity114Task> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity114Task] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity114Task> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}