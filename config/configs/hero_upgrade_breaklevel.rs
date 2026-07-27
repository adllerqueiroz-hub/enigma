// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroUpgradeBreaklevel {
    #[serde(rename = "skillId")]
    pub skill_id: i32,
    #[serde(rename = "upgradeSkillId")]
    pub upgrade_skill_id: i32,
}
pub struct HeroUpgradeBreaklevelTable {
    records: Vec<HeroUpgradeBreaklevel>,
}

impl HeroUpgradeBreaklevelTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<HeroUpgradeBreaklevel> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[HeroUpgradeBreaklevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, HeroUpgradeBreaklevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}