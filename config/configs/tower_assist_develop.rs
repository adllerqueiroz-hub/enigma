// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerAssistDevelop {
    pub attribute: String,
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    #[serde(rename = "extraRule")]
    pub extra_rule: String,
    pub level: i32,
    #[serde(rename = "passiveSkills")]
    pub passive_skills: String,
    #[serde(rename = "talentPoint")]
    pub talent_point: i32,
}
pub struct TowerAssistDevelopTable {
    records: Vec<TowerAssistDevelop>,
}

impl TowerAssistDevelopTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerAssistDevelop> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TowerAssistDevelop] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerAssistDevelop> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}