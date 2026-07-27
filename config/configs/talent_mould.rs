// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentMould {
    #[serde(rename = "allShape")]
    pub all_shape: String,
    #[serde(rename = "talentId")]
    pub talent_id: i32,
    #[serde(rename = "talentMould")]
    pub talent_mould: i32,
    pub type10: String,
    pub type11: String,
    pub type12: String,
    pub type13: String,
    pub type14: String,
    pub type15: String,
    pub type16: String,
    pub type17: String,
    pub type18: String,
    pub type19: String,
    pub type20: String,
}
pub struct TalentMouldTable {
    records: Vec<TalentMould>,
}

impl TalentMouldTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TalentMould> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TalentMould] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TalentMould> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}