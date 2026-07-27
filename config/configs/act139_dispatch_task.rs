// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act139DispatchTask {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub desc: String,
    #[serde(rename = "elementId")]
    pub element_id: i32,
    #[serde(rename = "extraParam")]
    pub extra_param: String,
    pub id: i32,
    pub image: String,
    #[serde(rename = "maxCount")]
    pub max_count: i32,
    #[serde(rename = "minCount")]
    pub min_count: i32,
    #[serde(rename = "shortType")]
    pub short_type: i32,
    pub time: String,
    pub title: String,
}
use std::collections::HashMap;

pub struct Act139DispatchTaskTable {
    records: Vec<Act139DispatchTask>,
    by_id: HashMap<i32, usize>,
}

impl Act139DispatchTaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Act139DispatchTask> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Act139DispatchTask> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Act139DispatchTask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Act139DispatchTask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}