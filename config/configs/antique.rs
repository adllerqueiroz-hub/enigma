// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Antique {
    pub desc: String,
    pub effect: String,
    pub gifticon: String,
    pub icon: String,
    #[serde(rename = "iconArea")]
    pub icon_area: i32,
    pub id: i32,
    pub name: String,
    pub nameen: String,
    pub sign: String,
    pub sources: String,
    #[serde(rename = "storyId")]
    pub story_id: i32,
    pub title: String,
    pub titleen: String,
}
use std::collections::HashMap;

pub struct AntiqueTable {
    records: Vec<Antique>,
    by_id: HashMap<i32, usize>,
}

impl AntiqueTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Antique> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Antique> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Antique] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Antique> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}