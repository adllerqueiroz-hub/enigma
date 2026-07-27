// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity154Options {
    #[serde(rename = "optionId")]
    pub option_id: i32,
    #[serde(rename = "optionText")]
    pub option_text: String,
    #[serde(rename = "puzzleId")]
    pub puzzle_id: i32,
}
pub struct Activity154OptionsTable {
    records: Vec<Activity154Options>,
}

impl Activity154OptionsTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity154Options> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity154Options] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity154Options> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}