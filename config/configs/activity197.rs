// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity197 {
    #[serde(rename = "activityConsume")]
    pub activity_consume: i32,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "consumeBonus")]
    pub consume_bonus: String,
    #[serde(rename = "doubleTimes")]
    pub double_times: i32,
    #[serde(rename = "exploreConsume")]
    pub explore_consume: String,
    #[serde(rename = "exploreItem")]
    pub explore_item: String,
    #[serde(rename = "exploreNum")]
    pub explore_num: String,
    #[serde(rename = "rummageConsume")]
    pub rummage_consume: String,
}
pub struct Activity197Table {
    records: Vec<Activity197>,
}

impl Activity197Table {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity197> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity197] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity197> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}