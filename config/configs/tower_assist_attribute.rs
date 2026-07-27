// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerAssistAttribute {
    pub attack: i32,
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    pub cri: i32,
    #[serde(rename = "criDmg")]
    pub cri_dmg: i32,
    pub hp: i32,
    #[serde(rename = "teamLevel")]
    pub team_level: i32,
}
pub struct TowerAssistAttributeTable {
    records: Vec<TowerAssistAttribute>,
}

impl TowerAssistAttributeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerAssistAttribute> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TowerAssistAttribute] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerAssistAttribute> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}