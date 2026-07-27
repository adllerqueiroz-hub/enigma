// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerPermanentTime {
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "stageId")]
    pub stage_id: i32,
    pub time: String,
}
pub struct TowerPermanentTimeTable {
    records: Vec<TowerPermanentTime>,
}

impl TowerPermanentTimeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerPermanentTime> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TowerPermanentTime] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerPermanentTime> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}