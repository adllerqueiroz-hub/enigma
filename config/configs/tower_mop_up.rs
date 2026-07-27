// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerMopUp {
    pub id: i32,
    #[serde(rename = "layerNum")]
    pub layer_num: i32,
    pub reward: String,
}
use std::collections::HashMap;

pub struct TowerMopUpTable {
    records: Vec<TowerMopUp>,
    by_id: HashMap<i32, usize>,
}

impl TowerMopUpTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerMopUp> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&TowerMopUp> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[TowerMopUp] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerMopUp> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}