// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity225RedEnvelopeRainPool {
    pub source: String,
}
pub struct Activity225RedEnvelopeRainPoolTable {
    records: Vec<Activity225RedEnvelopeRainPool>,
}

impl Activity225RedEnvelopeRainPoolTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity225RedEnvelopeRainPool> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity225RedEnvelopeRainPool] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity225RedEnvelopeRainPool> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}