// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDestinyFacets {
    pub desc: String,
    #[serde(rename = "deviceAdd")]
    pub device_add: String,
    pub ex_level_exchange: bool,
    #[serde(rename = "exchangeSkills")]
    pub exchange_skills: String,
    #[serde(rename = "facetsId")]
    pub facets_id: i32,
    pub level: i32,
    #[serde(rename = "powerAdd")]
    pub power_add: String,
    #[serde(rename = "uniqueSkill_point")]
    pub unique_skill_point: String,
}
pub struct CharacterDestinyFacetsTable {
    records: Vec<CharacterDestinyFacets>,
}

impl CharacterDestinyFacetsTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<CharacterDestinyFacets> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[CharacterDestinyFacets] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CharacterDestinyFacets> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}