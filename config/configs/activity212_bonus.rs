// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity212Bonus {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    pub id: i32,
    #[serde(rename = "packsId")]
    pub packs_id: i32,
}
use std::collections::HashMap;

pub struct Activity212BonusTable {
    records: Vec<Activity212Bonus>,
    by_id: HashMap<i32, usize>,
}

impl Activity212BonusTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity212Bonus> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Activity212Bonus> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Activity212Bonus] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity212Bonus> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}