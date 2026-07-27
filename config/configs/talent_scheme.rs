// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentScheme {
    #[serde(rename = "starMould")]
    pub star_mould: i32,
    #[serde(rename = "talenScheme")]
    pub talen_scheme: String,
    #[serde(rename = "talentId")]
    pub talent_id: i32,
    #[serde(rename = "talentMould")]
    pub talent_mould: i32,
}
pub struct TalentSchemeTable {
    records: Vec<TalentScheme>,
}

impl TalentSchemeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TalentScheme> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TalentScheme] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TalentScheme> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}