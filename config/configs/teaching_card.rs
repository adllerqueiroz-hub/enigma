// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeachingCard {
    pub id: i32,
    #[serde(rename = "openingCards")]
    pub opening_cards: String,
    #[serde(rename = "refillCards")]
    pub refill_cards: String,
}
use std::collections::HashMap;

pub struct TeachingCardTable {
    records: Vec<TeachingCard>,
    by_id: HashMap<i32, usize>,
}

impl TeachingCardTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TeachingCard> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&TeachingCard> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[TeachingCard] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TeachingCard> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}