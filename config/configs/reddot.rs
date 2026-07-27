// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reddot {
    #[serde(rename = "canLoad")]
    pub can_load: i32,
    pub id: i32,
    #[serde(rename = "isOnline")]
    pub is_online: i32,
    pub parent: String,
    pub style: i32,
}
use std::collections::HashMap;

pub struct ReddotTable {
    records: Vec<Reddot>,
    by_id: HashMap<i32, usize>,
}

impl ReddotTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Reddot> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Reddot> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Reddot] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Reddot> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}