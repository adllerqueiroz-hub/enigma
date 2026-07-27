// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardGroup {
    pub count: String,
    pub group: String,
    pub id: i32,
    pub label: String,
    #[serde(rename = "materialId")]
    pub material_id: i32,
    #[serde(rename = "materialType")]
    pub material_type: i32,
    pub shownum: i32,
}
use std::collections::HashMap;

pub struct RewardGroupTable {
    records: Vec<RewardGroup>,
    by_id: HashMap<i32, usize>,
}

impl RewardGroupTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<RewardGroup> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&RewardGroup> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[RewardGroup] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RewardGroup> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}