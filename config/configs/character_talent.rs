// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTalent {
    pub consume: String,
    pub exclusive: String,
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    pub requirement: i32,
    #[serde(rename = "talentId")]
    pub talent_id: i32,
    #[serde(rename = "talentMould")]
    pub talent_mould: i32,
}
pub struct CharacterTalentTable {
    records: Vec<CharacterTalent>,
}

impl CharacterTalentTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<CharacterTalent> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[CharacterTalent] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CharacterTalent> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}