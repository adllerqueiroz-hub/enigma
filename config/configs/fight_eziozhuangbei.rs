// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightEziozhuangbei {
    #[serde(rename = "exchangeSkills")]
    pub exchange_skills: String,
    #[serde(rename = "firstDesc")]
    pub first_desc: String,
    #[serde(rename = "firstId")]
    pub first_id: i32,
    #[serde(rename = "passiveSkill")]
    pub passive_skill: String,
    #[serde(rename = "secondDesc")]
    pub second_desc: String,
    #[serde(rename = "secondId")]
    pub second_id: i32,
    #[serde(rename = "skillEx")]
    pub skill_ex: i32,
    #[serde(rename = "skillGroup1")]
    pub skill_group1: String,
    #[serde(rename = "skillGroup2")]
    pub skill_group2: String,
    #[serde(rename = "skillLevel")]
    pub skill_level: i32,
}
pub struct FightEziozhuangbeiTable {
    records: Vec<FightEziozhuangbei>,
}

impl FightEziozhuangbeiTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<FightEziozhuangbei> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[FightEziozhuangbei] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightEziozhuangbei> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}