// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    #[serde(rename = "dailyDrop")]
    pub daily_drop: i32,
    #[serde(rename = "dailyGainWarning")]
    pub daily_gain_warning: i32,
    #[serde(rename = "rewardGroup1")]
    pub reward_group1: String,
    #[serde(rename = "rewardGroup2")]
    pub reward_group2: String,
    #[serde(rename = "rewardGroup3")]
    pub reward_group3: String,
    #[serde(rename = "rewardGroup4")]
    pub reward_group4: String,
    #[serde(rename = "rewardGroup5")]
    pub reward_group5: String,
    #[serde(rename = "rewardGroup6")]
    pub reward_group6: String,
    #[serde(rename = "rewardGroup7")]
    pub reward_group7: String,
    #[serde(rename = "rewardGroup8")]
    pub reward_group8: String,
    pub reward_id: i32,
}
pub struct RewardTable {
    records: Vec<Reward>,
}

impl RewardTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Reward> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Reward] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Reward> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}