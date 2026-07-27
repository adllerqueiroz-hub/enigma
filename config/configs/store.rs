// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Store {
    #[serde(rename = "autoRefreshTime")]
    pub auto_refresh_time: String,
    pub id: i32,
    #[serde(rename = "needClearStore")]
    pub need_clear_store: i32,
    #[serde(rename = "refreshCost")]
    pub refresh_cost: String,
}
use std::collections::HashMap;

pub struct StoreTable {
    records: Vec<Store>,
    by_id: HashMap<i32, usize>,
}

impl StoreTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Store> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Store> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Store] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Store> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}