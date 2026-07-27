// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity131Task {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    pub id: i32,
    #[serde(rename = "isOnline")]
    pub is_online: i32,
    #[serde(rename = "jumpId")]
    pub jump_id: i32,
    #[serde(rename = "listenerParam")]
    pub listener_param: String,
    #[serde(rename = "listenerType")]
    pub listener_type: String,
    #[serde(rename = "loopType")]
    pub loop_type: String,
    #[serde(rename = "maxProgress")]
    pub max_progress: i32,
    pub name: String,
}
use std::collections::HashMap;

pub struct Activity131TaskTable {
    records: Vec<Activity131Task>,
    by_id: HashMap<i32, usize>,
}

impl Activity131TaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity131Task> = crate::load_rows(path)?;

        let mut by_id = HashMap::with_capacity(records.len());

        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }

        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Activity131Task> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Activity131Task] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity131Task> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}