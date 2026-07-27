// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementTask {
    #[serde(rename = "achievementId")]
    pub achievement_id: i32,
    pub desc: String,
    #[serde(rename = "extraDesc")]
    pub extra_desc: String,
    pub icon: String,
    pub id: i32,
    pub image: String,
    pub level: i32,
    #[serde(rename = "listenerParam")]
    pub listener_param: String,
    #[serde(rename = "listenerType")]
    pub listener_type: String,
    #[serde(rename = "maxProgress")]
    pub max_progress: i32,
    #[serde(rename = "sortId")]
    pub sort_id: i32,
}
use std::collections::HashMap;

pub struct AchievementTaskTable {
    records: Vec<AchievementTask>,
    by_id: HashMap<i32, usize>,
}

impl AchievementTaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<AchievementTask> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&AchievementTask> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[AchievementTask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, AchievementTask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}