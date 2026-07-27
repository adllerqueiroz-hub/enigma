// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerAction {
    #[serde(rename = "actionType")]
    pub action_type: String,
    pub id: i32,
    pub param1: String,
    pub param10: String,
    pub param11: String,
    pub param12: String,
    pub param13: String,
    pub param14: String,
    pub param15: String,
    pub param2: String,
    pub param3: String,
    pub param4: String,
    pub param5: String,
    pub param6: String,
    pub param7: String,
    pub param8: String,
    pub param9: String,
}
use std::collections::HashMap;

pub struct TriggerActionTable {
    records: Vec<TriggerAction>,
    by_id: HashMap<i32, usize>,
}

impl TriggerActionTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TriggerAction> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&TriggerAction> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[TriggerAction] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TriggerAction> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}