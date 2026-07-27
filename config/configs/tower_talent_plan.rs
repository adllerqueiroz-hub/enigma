// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerTalentPlan {
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    #[serde(rename = "planId")]
    pub plan_id: i32,
    #[serde(rename = "planName")]
    pub plan_name: String,
    #[serde(rename = "talentIds")]
    pub talent_ids: String,
}
pub struct TowerTalentPlanTable {
    records: Vec<TowerTalentPlan>,
}

impl TowerTalentPlanTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerTalentPlan> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TowerTalentPlan] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerTalentPlan> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}