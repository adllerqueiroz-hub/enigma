// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166Base {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "baseId")]
    pub base_id: i32,
    pub desc: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    pub level: i32,
    pub name: String,
    pub strategy: String,
    #[serde(rename = "talentId")]
    pub talent_id: i32,
}
pub struct Activity166BaseTable {
    records: Vec<Activity166Base>,
}

impl Activity166BaseTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity166Base> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity166Base] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166Base> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}