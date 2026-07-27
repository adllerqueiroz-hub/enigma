// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity146 {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    pub id: i32,
    #[serde(rename = "interactType")]
    pub interact_type: i32,
    pub name: String,
    #[serde(rename = "openDay")]
    pub open_day: i32,
    pub photo: String,
    #[serde(rename = "preId")]
    pub pre_id: i32,
    pub text: String,
}
use std::collections::HashMap;

pub struct Activity146Table {
    records: Vec<Activity146>,
    by_id: HashMap<i32, usize>,
}

impl Activity146Table {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity146> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Activity146> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Activity146] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity146> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}