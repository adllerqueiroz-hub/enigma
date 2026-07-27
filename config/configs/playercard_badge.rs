// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayercardBadge {
    pub group: i32,
    pub icon: String,
    pub id: i32,
    pub level: i32,
    pub name: String,
    #[serde(rename = "taskId")]
    pub task_id: i32,
}
use std::collections::HashMap;

pub struct PlayercardBadgeTable {
    records: Vec<PlayercardBadge>,
    by_id: HashMap<i32, usize>,
}

impl PlayercardBadgeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<PlayercardBadge> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&PlayercardBadge> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[PlayercardBadge] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, PlayercardBadge> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}