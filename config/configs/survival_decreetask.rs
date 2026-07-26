// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalDecreetask {
    pub desc: String,
    pub desc2: i32,
    #[serde(rename = "failCondition")]
    pub fail_condition: i32,
    pub group: i32,
    pub id: i32,
    #[serde(rename = "maxProgress")]
    pub max_progress: String,
    #[serde(rename = "needAccept")]
    pub need_accept: String,
    pub prepose: String,
    #[serde(rename = "progressCondition")]
    pub progress_condition: i32,
    pub seasons: String,
    pub step: String,
    pub tag: String,
    pub versions: String,
}
use std::collections::HashMap;

pub struct SurvivalDecreetaskTable {
    records: Vec<SurvivalDecreetask>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalDecreetaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<SurvivalDecreetask> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalDecreetask> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalDecreetask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalDecreetask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}