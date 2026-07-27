// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDestiny {
    #[serde(rename = "facetsId")]
    pub facets_id: String,
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    #[serde(rename = "slotsId")]
    pub slots_id: i32,
}
pub struct CharacterDestinyTable {
    records: Vec<CharacterDestiny>,
}

impl CharacterDestinyTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<CharacterDestiny> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[CharacterDestiny] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CharacterDestiny> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}