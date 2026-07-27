// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actvity186Stage {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "globalTaskActivityId")]
    pub global_task_activity_id: i32,
    #[serde(rename = "globalTaskId")]
    pub global_task_id: i32,
    #[serde(rename = "stageId")]
    pub stage_id: i32,
    #[serde(rename = "startTime")]
    pub start_time: String,
}
pub struct Actvity186StageTable {
    records: Vec<Actvity186Stage>,
}

impl Actvity186StageTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Actvity186Stage> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Actvity186Stage] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actvity186Stage> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}