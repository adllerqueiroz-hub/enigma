// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rouge2Material {
    pub details: String,
    pub effects: String,
    pub formula: String,
    pub icon: String,
    pub id: i32,
    pub name: String,
    #[serde(rename = "numMax")]
    pub num_max: i32,
    pub rare: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}
use std::collections::HashMap;

pub struct Rouge2MaterialTable {
    records: Vec<Rouge2Material>,
    by_id: HashMap<i32, usize>,
}

impl Rouge2MaterialTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Rouge2Material> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Rouge2Material> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Rouge2Material] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rouge2Material> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}