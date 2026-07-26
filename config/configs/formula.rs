// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Formula {
    #[serde(rename = "costMaterial")]
    pub cost_material: String,
    #[serde(rename = "costReserve")]
    pub cost_reserve: i32,
    #[serde(rename = "costScore")]
    pub cost_score: String,
    #[serde(rename = "costTime")]
    pub cost_time: i32,
    pub desc: String,
    pub icon: String,
    pub id: i32,
    pub name: String,
    #[serde(rename = "needEpisodeId")]
    pub need_episode_id: i32,
    #[serde(rename = "needProductionLevel")]
    pub need_production_level: i32,
    #[serde(rename = "needRoomLevel")]
    pub need_room_level: i32,
    pub order: i32,
    pub produce: String,
    pub rare: i32,
    #[serde(rename = "showType")]
    pub show_type: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "useDesc")]
    pub use_desc: String,
}
use std::collections::HashMap;

pub struct FormulaTable {
    records: Vec<Formula>,
    by_id: HashMap<i32, usize>,
}

impl FormulaTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<Formula> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Formula> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Formula] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Formula> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}