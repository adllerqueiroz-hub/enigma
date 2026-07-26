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
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<Actvity205MiniGameReward> = if let Some(array) = value.as_array() {
            if array.len() >= 2 && array[1].is_array() {
                serde_json::from_value(array[1].clone())?
            } else {
                serde_json::from_value(value)?
            }
        } else {
            serde_json::from_value(value)?
        };

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