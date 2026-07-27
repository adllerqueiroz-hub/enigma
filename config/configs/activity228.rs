// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity228 {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub column: i32,
    pub cost: String,
    #[serde(rename = "effectWeights")]
    pub effect_weights: String,
    #[serde(rename = "finalReward")]
    pub final_reward: String,
    #[serde(rename = "intensifyGuaranteeCount")]
    pub intensify_guarantee_count: i32,
    #[serde(rename = "intensifyRate")]
    pub intensify_rate: i32,
    pub reward: String,
    pub row: i32,
}
pub struct Activity228Table {
    records: Vec<Activity228>,
}

impl Activity228Table {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity228> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity228] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity228> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}