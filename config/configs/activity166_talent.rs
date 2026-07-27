// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166Talent {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "baseSkillIds")]
    pub base_skill_ids: String,
    #[serde(rename = "baseSkillIds2")]
    pub base_skill_ids2: String,
    pub icon: String,
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "sortIndex")]
    pub sort_index: i32,
    #[serde(rename = "talentId")]
    pub talent_id: i32,
}
pub struct Activity166TalentTable {
    records: Vec<Activity166Talent>,
}

impl Activity166TalentTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity166Talent> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity166Talent] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166Talent> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}