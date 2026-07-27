// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterLevel {
    pub add_dmg: i32,
    pub atk: i32,
    pub cri: i32,
    pub cri_def: i32,
    pub cri_dmg: i32,
    pub def: i32,
    pub drop_dmg: i32,
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    pub hp: i32,
    pub level: i32,
    pub mdef: i32,
    pub recri: i32,
    pub technic: i32,
}
pub struct CharacterLevelTable {
    records: Vec<CharacterLevel>,
}

impl CharacterLevelTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<CharacterLevel> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[CharacterLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CharacterLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}