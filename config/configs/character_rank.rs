// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRank {
    pub consume: String,
    pub effect: String,
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    pub rank: i32,
    pub requirement: String,
}
pub struct CharacterRankTable {
    records: Vec<CharacterRank>,
}

impl CharacterRankTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<CharacterRank> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[CharacterRank] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CharacterRank> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}