// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166TalentStyle {
    pub level: i32,
    #[serde(rename = "needStar")]
    pub need_star: i32,
    #[serde(rename = "skillId")]
    pub skill_id: String,
    #[serde(rename = "skillId2")]
    pub skill_id2: String,
    pub slot: i32,
    #[serde(rename = "talentId")]
    pub talent_id: i32,
}
pub struct Activity166TalentStyleTable {
    records: Vec<Activity166TalentStyle>,
}

impl Activity166TalentStyleTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity166TalentStyle> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity166TalentStyle] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166TalentStyle> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}