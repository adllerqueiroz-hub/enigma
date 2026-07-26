// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnbackTaskBonus {
    pub bonus: String,
    pub character: i32,
    pub content: String,
    #[serde(rename = "extraBonus")]
    pub extra_bonus: String,
    pub id: i32,
    #[serde(rename = "needPoint")]
    pub need_point: i32,
    #[serde(rename = "turnbackId")]
    pub turnback_id: i32,
}
use std::collections::HashMap;

pub struct TurnbackTaskBonusTable {
    records: Vec<TurnbackTaskBonus>,
    by_id: HashMap<i32, usize>,
}

impl TurnbackTaskBonusTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<TurnbackTaskBonus> = if let Some(array) = value.as_array() {
            if array.len() >= 2 && array[1].is_array() {
                serde_json::from_value(array[1].clone())?
            } else {
                serde_json::from_value(value)?
            }
        } else {
            serde_json::from_value(value)?
        };

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
    pub fn get(&self, id: i32) -> Option<&TurnbackTaskBonus> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[TurnbackTaskBonus] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TurnbackTaskBonus> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}