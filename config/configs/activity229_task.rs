// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity229Task {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    pub desc: String,
    pub id: i32,
    #[serde(rename = "isOnline")]
    pub is_online: i32,
    #[serde(rename = "listenerParam")]
    pub listener_param: String,
    #[serde(rename = "listenerType")]
    pub listener_type: String,
    #[serde(rename = "maxFinishCount")]
    pub max_finish_count: i32,
    #[serde(rename = "maxProgress")]
    pub max_progress: i32,
    #[serde(rename = "minType")]
    pub min_type: String,
    pub name: String,
}
use std::collections::HashMap;

pub struct Activity229TaskTable {
    records: Vec<Activity229Task>,
    by_id: HashMap<i32, usize>,
}

impl Activity229TaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity229Task> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Activity229Task> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Activity229Task] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity229Task> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}