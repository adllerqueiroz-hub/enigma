// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeachingSummon {
    #[serde(rename = "grantGuideId")]
    pub grant_guide_id: i32,
    #[serde(rename = "grantReward")]
    pub grant_reward: String,
    #[serde(rename = "grantStepId")]
    pub grant_step_id: i32,
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    pub id: i32,
    #[serde(rename = "poolId")]
    pub pool_id: i32,
    #[serde(rename = "previousStepId")]
    pub previous_step_id: i32,
    #[serde(rename = "stepId")]
    pub step_id: i32,
}
use std::collections::HashMap;

pub struct TeachingSummonTable {
    records: Vec<TeachingSummon>,
    by_id: HashMap<i32, usize>,
}

impl TeachingSummonTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TeachingSummon> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&TeachingSummon> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[TeachingSummon] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TeachingSummon> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}