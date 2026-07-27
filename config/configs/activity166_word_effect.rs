// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166WordEffect {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub desc: String,
    pub id: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}
use std::collections::HashMap;

pub struct Activity166WordEffectTable {
    records: Vec<Activity166WordEffect>,
    by_id: HashMap<i32, usize>,
}

impl Activity166WordEffectTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity166WordEffect> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Activity166WordEffect> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Activity166WordEffect] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166WordEffect> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}