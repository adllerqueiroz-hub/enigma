// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuffAct {
    #[serde(rename = "audioId")]
    pub audio_id: i32,
    pub effect: String,
    #[serde(rename = "effectCondition")]
    pub effect_condition: i32,
    #[serde(rename = "effectHangPoint")]
    pub effect_hang_point: String,
    #[serde(rename = "effectTime")]
    pub effect_time: i32,
    pub id: i32,
    #[serde(rename = "type")]
    pub r#type: String,
}
use std::collections::HashMap;

pub struct BuffActTable {
    records: Vec<BuffAct>,
    by_id: HashMap<i32, usize>,
}

impl BuffActTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<BuffAct> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&BuffAct> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[BuffAct] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, BuffAct> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}