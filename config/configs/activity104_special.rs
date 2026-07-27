// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity104Special {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub desc: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    pub icon: String,
    pub layer: i32,
    pub level: i32,
    pub name: String,
    pub nameen: String,
    #[serde(rename = "openDay")]
    pub open_day: i32,
}
pub struct Activity104SpecialTable {
    records: Vec<Activity104Special>,
}

impl Activity104SpecialTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity104Special> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity104Special] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity104Special> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}