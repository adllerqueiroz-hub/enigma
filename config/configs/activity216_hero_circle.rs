// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity216HeroCircle {
    #[serde(rename = "heroId")]
    pub hero_id: String,
    pub id: i32,
    #[serde(rename = "type")]
    pub r#type: String,
}
use std::collections::HashMap;

pub struct Activity216HeroCircleTable {
    records: Vec<Activity216HeroCircle>,
    by_id: HashMap<i32, usize>,
}

impl Activity216HeroCircleTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity216HeroCircle> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Activity216HeroCircle> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Activity216HeroCircle] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity216HeroCircle> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}