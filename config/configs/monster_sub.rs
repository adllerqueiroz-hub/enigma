// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterSub {
    pub attack: i32,
    pub defense: i32,
    pub id: i32,
    pub job: i32,
    pub life: i32,
    pub mdefense: i32,
    pub score: String,
    pub technic: i32,
}
use std::collections::HashMap;

pub struct MonsterSubTable {
    records: Vec<MonsterSub>,
    by_id: HashMap<i32, usize>,
}

impl MonsterSubTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<MonsterSub> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&MonsterSub> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[MonsterSub] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, MonsterSub> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}