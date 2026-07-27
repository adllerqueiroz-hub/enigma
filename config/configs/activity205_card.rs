// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity205Card {
    pub desc: String,
    pub id: i32,
    pub img: String,
    pub name: String,
    pub restrain: String,
    #[serde(rename = "spEff")]
    pub sp_eff: i32,
    #[serde(rename = "subWeight")]
    pub sub_weight: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
    pub weight: i32,
}
use std::collections::HashMap;

pub struct Activity205CardTable {
    records: Vec<Activity205Card>,
    by_id: HashMap<i32, usize>,
}

impl Activity205CardTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity205Card> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Activity205Card> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Activity205Card] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity205Card> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}