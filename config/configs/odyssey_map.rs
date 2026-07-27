// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdysseyMap {
    pub id: i32,
    #[serde(rename = "initPos")]
    pub init_pos: String,
    #[serde(rename = "mapName")]
    pub map_name: String,
    #[serde(rename = "recommendLevel")]
    pub recommend_level: String,
    pub res: String,
    #[serde(rename = "unlockCondition")]
    pub unlock_condition: String,
    #[serde(rename = "unlockDesc")]
    pub unlock_desc: String,
}
use std::collections::HashMap;

pub struct OdysseyMapTable {
    records: Vec<OdysseyMap>,
    by_id: HashMap<i32, usize>,
}

impl OdysseyMapTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<OdysseyMap> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&OdysseyMap> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[OdysseyMap] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, OdysseyMap> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}