// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterCosume {
    pub cosume: String,
    pub level: i32,
    pub rare: i32,
}
pub struct CharacterCosumeTable {
    records: Vec<CharacterCosume>,
}

impl CharacterCosumeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<CharacterCosume> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[CharacterCosume] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CharacterCosume> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}