// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rouge2Reward {
    pub group: i32,
    pub id: i32,
    #[serde(rename = "maxBuyCount")]
    pub max_buy_count: i32,
    pub num: i32,
    #[serde(rename = "rewardImage")]
    pub reward_image: String,
    #[serde(rename = "rewardPriority")]
    pub reward_priority: i32,
    #[serde(rename = "rewardScore")]
    pub reward_score: i32,
    pub stage: i32,
    pub value: String,
}
use std::collections::HashMap;

pub struct Rouge2RewardTable {
    records: Vec<Rouge2Reward>,
    by_id: HashMap<i32, usize>,
}

impl Rouge2RewardTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Rouge2Reward> = crate::load_rows(path)?;

        let mut by_id = HashMap::with_capacity(records.len());

        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }

        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Rouge2Reward> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Rouge2Reward] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rouge2Reward> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}