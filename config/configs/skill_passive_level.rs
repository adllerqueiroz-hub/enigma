// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPassiveLevel {
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    #[serde(rename = "skillGroup")]
    pub skill_group: i32,
    #[serde(rename = "skillLevel")]
    pub skill_level: i32,
    #[serde(rename = "skillPassive")]
    pub skill_passive: i32,
    #[serde(rename = "uiFilterSkill")]
    pub ui_filter_skill: String,
}
pub struct SkillPassiveLevelTable {
    records: Vec<SkillPassiveLevel>,
}

impl SkillPassiveLevelTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<SkillPassiveLevel> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[SkillPassiveLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SkillPassiveLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}