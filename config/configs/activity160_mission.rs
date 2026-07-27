// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity160Mission {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    pub desc: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    pub id: i32,
    #[serde(rename = "mailId")]
    pub mail_id: i32,
    #[serde(rename = "preId")]
    pub pre_id: i32,
    pub sort: i32,
}
use std::collections::HashMap;

pub struct Activity160MissionTable {
    records: Vec<Activity160Mission>,
    by_id: HashMap<i32, usize>,
}

impl Activity160MissionTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity160Mission> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Activity160Mission> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Activity160Mission] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity160Mission> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}