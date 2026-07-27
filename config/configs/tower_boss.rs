// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerBoss {
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "towerId")]
    pub tower_id: i32,
}
pub struct TowerBossTable {
    records: Vec<TowerBoss>,
}

impl TowerBossTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerBoss> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TowerBoss] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerBoss> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}