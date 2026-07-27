// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExLevel {
    pub consume: String,
    pub consume2: String,
    pub desc: String,
    #[serde(rename = "deviceId")]
    pub device_id: i32,
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    #[serde(rename = "passiveSkill")]
    pub passive_skill: String,
    pub requirement: String,
    #[serde(rename = "skillEx")]
    pub skill_ex: i32,
    #[serde(rename = "skillGroup1")]
    pub skill_group1: String,
    #[serde(rename = "skillGroup2")]
    pub skill_group2: String,
    #[serde(rename = "skillLevel")]
    pub skill_level: i32,
}
pub struct SkillExLevelTable {
    records: Vec<SkillExLevel>,
}

impl SkillExLevelTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<SkillExLevel> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[SkillExLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SkillExLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}