// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act139SubHeroTask {
    pub desc: String,
    #[serde(rename = "descSuffix")]
    pub desc_suffix: String,
    #[serde(rename = "elementIds")]
    pub element_ids: String,
    pub id: i32,
    pub image: String,
    #[serde(rename = "lockDesc")]
    pub lock_desc: String,
    pub reward: String,
    #[serde(rename = "storyId")]
    pub story_id: i32,
    #[serde(rename = "taskId")]
    pub task_id: i32,
    pub title: String,
    #[serde(rename = "unlockParam")]
    pub unlock_param: String,
    #[serde(rename = "unlockType")]
    pub unlock_type: i32,
}
use std::collections::HashMap;

pub struct Act139SubHeroTaskTable {
    records: Vec<Act139SubHeroTask>,
    by_id: HashMap<i32, usize>,
}

impl Act139SubHeroTaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<Act139SubHeroTask> = if let Some(array) = value.as_array() {
            if array.len() >= 2 && array[1].is_array() {
                serde_json::from_value(array[1].clone())?
            } else {
                serde_json::from_value(value)?
            }
        } else {
            serde_json::from_value(value)?
        };

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
    pub fn get(&self, id: i32) -> Option<&Act139SubHeroTask> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Act139SubHeroTask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Act139SubHeroTask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}