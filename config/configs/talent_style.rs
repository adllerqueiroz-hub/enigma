// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentStyle {
    pub color: String,
    pub level: i32,
    pub name: String,
    #[serde(rename = "replaceCube")]
    pub replace_cube: String,
    #[serde(rename = "styleId")]
    pub style_id: i32,
    pub tag: String,
    pub tagicon: String,
    #[serde(rename = "talentMould")]
    pub talent_mould: i32,
}
pub struct TalentStyleTable {
    records: Vec<TalentStyle>,
}

impl TalentStyleTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TalentStyle> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TalentStyle] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TalentStyle> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}