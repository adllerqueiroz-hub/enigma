// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166InfoBonus {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "analyCount")]
    pub analy_count: i32,
    pub bonus: String,
}
pub struct Activity166InfoBonusTable {
    records: Vec<Activity166InfoBonus>,
}

impl Activity166InfoBonusTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity166InfoBonus> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity166InfoBonus] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166InfoBonus> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}