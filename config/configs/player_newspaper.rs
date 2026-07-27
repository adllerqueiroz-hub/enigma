// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerNewspaper {
    #[serde(rename = "displayPriority")]
    pub display_priority: i32,
    pub id: i32,
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "type")]
    pub r#type: i32,
}
use std::collections::HashMap;

pub struct PlayerNewspaperTable {
    records: Vec<PlayerNewspaper>,
    by_id: HashMap<i32, usize>,
}

impl PlayerNewspaperTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<PlayerNewspaper> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&PlayerNewspaper> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[PlayerNewspaper] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, PlayerNewspaper> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}