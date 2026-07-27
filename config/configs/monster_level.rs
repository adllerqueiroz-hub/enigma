// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterLevel {
    pub base: i32,
    pub base_super: i32,
    pub equip_base: i32,
    pub equip_super: i32,
    pub id: i32,
    pub technic: i32,
}
use std::collections::HashMap;

pub struct MonsterLevelTable {
    records: Vec<MonsterLevel>,
    by_id: HashMap<i32, usize>,
}

impl MonsterLevelTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<MonsterLevel> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&MonsterLevel> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[MonsterLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, MonsterLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}