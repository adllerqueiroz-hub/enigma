// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FairylandPuzzleTalk {
    #[serde(rename = "audioId")]
    pub audio_id: i32,
    pub content: String,
    #[serde(rename = "elementId")]
    pub element_id: i32,
    pub id: i32,
    pub param: String,
    pub speaker: String,
    pub step: i32,
    #[serde(rename = "type")]
    pub r#type: String,
}
use std::collections::HashMap;

pub struct FairylandPuzzleTalkTable {
    records: Vec<FairylandPuzzleTalk>,
    by_id: HashMap<i32, usize>,
}

impl FairylandPuzzleTalkTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<FairylandPuzzleTalk> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&FairylandPuzzleTalk> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[FairylandPuzzleTalk] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FairylandPuzzleTalk> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}