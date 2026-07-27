// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskActivityBonus {
    pub bonus: String,
    pub desc: String,
    #[serde(rename = "hideInVerifing")]
    pub hide_in_verifing: bool,
    pub id: i32,
    #[serde(rename = "needActivity")]
    pub need_activity: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}
use std::collections::HashMap;

pub struct TaskActivityBonusTable {
    records: Vec<TaskActivityBonus>,
    by_id: HashMap<i32, usize>,
}

impl TaskActivityBonusTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TaskActivityBonus> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&TaskActivityBonus> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[TaskActivityBonus] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TaskActivityBonus> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}