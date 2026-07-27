// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity206Reward {
    pub des: String,
    pub pic: String,
    pub reward: String,
    #[serde(rename = "rewardId")]
    pub reward_id: i32,
    pub title: String,
}
pub struct Activity206RewardTable {
    records: Vec<Activity206Reward>,
}

impl Activity206RewardTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity206Reward> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity206Reward] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity206Reward> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}