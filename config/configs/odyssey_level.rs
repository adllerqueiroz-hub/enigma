// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdysseyLevel {
    pub level: i32,
    #[serde(rename = "needExp")]
    pub need_exp: i32,
    pub reward: String,
}
pub struct OdysseyLevelTable {
    records: Vec<OdysseyLevel>,
}

impl OdysseyLevelTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<OdysseyLevel> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[OdysseyLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, OdysseyLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}