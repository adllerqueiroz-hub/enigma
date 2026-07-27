// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerHeroTrial {
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "heroIds")]
    pub hero_ids: String,
    pub season: i32,
    #[serde(rename = "startTime")]
    pub start_time: String,
}
pub struct TowerHeroTrialTable {
    records: Vec<TowerHeroTrial>,
}

impl TowerHeroTrialTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerHeroTrial> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TowerHeroTrial] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerHeroTrial> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}