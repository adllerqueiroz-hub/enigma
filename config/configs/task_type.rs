// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskType {
    #[serde(rename = "functuonId")]
    pub functuon_id: Vec<serde_json::Value>,
    pub id: i32,
    pub name: String,
    #[serde(rename = "redDotKey")]
    pub red_dot_key: i32,
}
use std::collections::HashMap;

pub struct TaskTypeTable {
    records: Vec<TaskType>,
    by_id: HashMap<i32, usize>,
}

impl TaskTypeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TaskType> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&TaskType> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[TaskType] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TaskType> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}