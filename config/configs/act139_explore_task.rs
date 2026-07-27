// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act139ExploreTask {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "areaPos")]
    pub area_pos: String,
    pub desc: String,
    #[serde(rename = "elementIds")]
    pub element_ids: String,
    pub id: i32,
    pub pos: String,
    #[serde(rename = "storyId")]
    pub story_id: i32,
    pub title: String,
    #[serde(rename = "titleEn")]
    pub title_en: String,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "unlockDesc")]
    pub unlock_desc: String,
    #[serde(rename = "unlockLineNumbers")]
    pub unlock_line_numbers: String,
    #[serde(rename = "unlockParam")]
    pub unlock_param: String,
    #[serde(rename = "unlockToastDesc")]
    pub unlock_toast_desc: String,
    #[serde(rename = "unlockType")]
    pub unlock_type: i32,
}
use std::collections::HashMap;

pub struct Act139ExploreTaskTable {
    records: Vec<Act139ExploreTask>,
    by_id: HashMap<i32, usize>,
}

impl Act139ExploreTaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Act139ExploreTask> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Act139ExploreTask> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Act139ExploreTask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Act139ExploreTask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}