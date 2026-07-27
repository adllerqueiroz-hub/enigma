// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166BaseLevel {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "baseId")]
    pub base_id: i32,
    #[serde(rename = "firstBonus")]
    pub first_bonus: String,
    pub level: i32,
}
pub struct Activity166BaseLevelTable {
    records: Vec<Activity166BaseLevel>,
}

impl Activity166BaseLevelTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity166BaseLevel> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity166BaseLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166BaseLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}