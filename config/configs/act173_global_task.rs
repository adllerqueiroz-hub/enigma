// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act173GlobalTask {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "endValue")]
    pub end_value: i32,
    pub id: i32,
    #[serde(rename = "isVisible")]
    pub is_visible: i32,
}
use std::collections::HashMap;

pub struct Act173GlobalTaskTable {
    records: Vec<Act173GlobalTask>,
    by_id: HashMap<i32, usize>,
}

impl Act173GlobalTaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Act173GlobalTask> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Act173GlobalTask> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Act173GlobalTask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Act173GlobalTask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}