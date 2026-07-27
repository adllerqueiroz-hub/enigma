// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity205Enter {
    pub desc: String,
    pub icon: String,
    pub id: i32,
    pub name: String,
    #[serde(rename = "targetDesc")]
    pub target_desc: String,
    pub times: i32,
}
use std::collections::HashMap;

pub struct Activity205EnterTable {
    records: Vec<Activity205Enter>,
    by_id: HashMap<i32, usize>,
}

impl Activity205EnterTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity205Enter> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Activity205Enter> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Activity205Enter] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity205Enter> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}