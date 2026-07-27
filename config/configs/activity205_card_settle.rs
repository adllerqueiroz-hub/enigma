// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity205CardSettle {
    pub desc: String,
    pub point: i32,
    #[serde(rename = "rewardId")]
    pub reward_id: i32,
}
pub struct Activity205CardSettleTable {
    records: Vec<Activity205CardSettle>,
}

impl Activity205CardSettleTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity205CardSettle> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity205CardSettle] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity205CardSettle> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}