// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerAssistBossChange {
    #[serde(rename = "activeSkills")]
    pub active_skills: String,
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    #[serde(rename = "coldTime")]
    pub cold_time: i32,
    pub form: i32,
    #[serde(rename = "passiveSkills")]
    pub passive_skills: String,
    #[serde(rename = "replacePassiveSkills")]
    pub replace_passive_skills: String,
    #[serde(rename = "resMaxVal")]
    pub res_max_val: i32,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
}
pub struct TowerAssistBossChangeTable {
    records: Vec<TowerAssistBossChange>,
}

impl TowerAssistBossChangeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerAssistBossChange> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TowerAssistBossChange] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerAssistBossChange> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}