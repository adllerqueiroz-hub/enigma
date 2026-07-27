// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity218MilestoneBonus {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    #[serde(rename = "coinNum")]
    pub coin_num: i32,
    #[serde(rename = "isSpBonus")]
    pub is_sp_bonus: bool,
    #[serde(rename = "rewardId")]
    pub reward_id: i32,
    pub source: String,
}
pub struct Activity218MilestoneBonusTable {
    records: Vec<Activity218MilestoneBonus>,
}

impl Activity218MilestoneBonusTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity218MilestoneBonus> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity218MilestoneBonus] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity218MilestoneBonus> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}