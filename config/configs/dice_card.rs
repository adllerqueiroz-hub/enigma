// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiceCard {
    pub aim1: i32,
    pub aim2: i32,
    pub aim3: i32,
    #[serde(rename = "allRoundLimitCount")]
    pub all_round_limit_count: i32,
    pub bufflist: String,
    pub desc: String,
    pub effect1: i32,
    pub effect2: i32,
    pub effect3: i32,
    pub id: i32,
    pub name: String,
    pub params1: String,
    pub params2: String,
    pub params3: String,
    pub patternlist: String,
    #[serde(rename = "powerExtendRule")]
    pub power_extend_rule: i32,
    pub quality: String,
    #[serde(rename = "roundLimitCount")]
    pub round_limit_count: i32,
    pub spiritskilltype: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}
use std::collections::HashMap;

pub struct DiceCardTable {
    records: Vec<DiceCard>,
    by_id: HashMap<i32, usize>,
}

impl DiceCardTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<DiceCard> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&DiceCard> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[DiceCard] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, DiceCard> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}