// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandbookCharacterAge {
    pub id: i32,
    pub image: String,
    pub order: i32,
    #[serde(rename = "rewardIcon")]
    pub reward_icon: String,
}
use std::collections::HashMap;

pub struct HandbookCharacterAgeTable {
    records: Vec<HandbookCharacterAge>,
    by_id: HashMap<i32, usize>,
}

impl HandbookCharacterAgeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<HandbookCharacterAge> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&HandbookCharacterAge> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[HandbookCharacterAge] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, HandbookCharacterAge> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}