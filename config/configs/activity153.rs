// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity153 {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "dailyLimit")]
    pub daily_limit: i32,
    #[serde(rename = "totalLimit")]
    pub total_limit: i32,
}
pub struct Activity153Table {
    records: Vec<Activity153>,
}

impl Activity153Table {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity153> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity153] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity153> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}