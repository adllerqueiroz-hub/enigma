// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    #[serde(rename = "actionList")]
    pub action_list: String,
    #[serde(rename = "battleId")]
    pub battle_id: i32,
    pub id: i32,
    pub limit: i32,
    #[serde(rename = "limitOneTurn")]
    pub limit_one_turn: i32,
    pub param1: String,
    pub param10: String,
    pub param2: String,
    pub param3: String,
    pub param4: String,
    pub param5: String,
    pub param6: String,
    pub param7: String,
    pub param8: String,
    pub param9: String,
    #[serde(rename = "triggerType")]
    pub trigger_type: String,
}
use std::collections::HashMap;

pub struct TriggerTable {
    records: Vec<Trigger>,
    by_id: HashMap<i32, usize>,
}

impl TriggerTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Trigger> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Trigger> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Trigger] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Trigger> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}