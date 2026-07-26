// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardEnchant {
    #[serde(rename = "coverType")]
    pub cover_type: String,
    #[serde(rename = "decStage")]
    pub dec_stage: i32,
    pub desc: String,
    #[serde(rename = "excludeTypes")]
    pub exclude_types: String,
    pub feature: String,
    pub id: i32,
    #[serde(rename = "rejectTypes")]
    pub reject_types: String,
    pub stage: i32,
}
use std::collections::HashMap;

pub struct CardEnchantTable {
    records: Vec<CardEnchant>,
    by_id: HashMap<i32, usize>,
}

impl CardEnchantTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<CardEnchant> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&CardEnchant> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[CardEnchant] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CardEnchant> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}