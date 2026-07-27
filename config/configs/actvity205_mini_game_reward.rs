// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actvity205MiniGameReward {
    pub bonus: String,
    #[serde(rename = "isWin")]
    pub is_win: bool,
    #[serde(rename = "rewardDesc")]
    pub reward_desc: String,
    #[serde(rename = "rewardId")]
    pub reward_id: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}
pub struct Actvity205MiniGameRewardTable {
    records: Vec<Actvity205MiniGameReward>,
}

impl Actvity205MiniGameRewardTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Actvity205MiniGameReward> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Actvity205MiniGameReward] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actvity205MiniGameReward> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}