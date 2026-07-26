// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalMaintask {
    #[serde(rename = "autoDrop")]
    pub auto_drop: i32,
    pub desc: String,
    pub desc2: String,
    #[serde(rename = "dropShow")]
    pub drop_show: String,
    #[serde(rename = "failCondition")]
    pub fail_condition: String,
    pub group: i32,
    pub id: i32,
    #[serde(rename = "maintaskChange")]
    pub maintask_change: String,
    #[serde(rename = "maxProgress")]
    pub max_progress: i32,
    #[serde(rename = "needAccept")]
    pub need_accept: i32,
    pub prepose: String,
    #[serde(rename = "progressCondition")]
    pub progress_condition: String,
    pub seasons: String,
    pub step: i32,
    pub versions: String,
}
use std::collections::HashMap;

pub struct SurvivalMaintaskTable {
    records: Vec<SurvivalMaintask>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalMaintaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<SurvivalMaintask> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalMaintask> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalMaintask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalMaintask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}