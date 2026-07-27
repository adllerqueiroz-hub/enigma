// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentStyleCost {
    pub consume: String,
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    #[serde(rename = "styleId")]
    pub style_id: i32,
}
pub struct TalentStyleCostTable {
    records: Vec<TalentStyleCost>,
}

impl TalentStyleCostTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TalentStyleCost> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TalentStyleCost] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TalentStyleCost> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}