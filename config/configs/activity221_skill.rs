// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity221Skill {
    pub cd: i32,
    pub condition: String,
    pub effects: String,
    #[serde(rename = "skillID")]
    pub skill_id: i32,
    #[serde(rename = "skillType")]
    pub skill_type: i32,
    pub skilldesc: String,
}
pub struct Activity221SkillTable {
    records: Vec<Activity221Skill>,
}

impl Activity221SkillTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity221Skill> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity221Skill] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity221Skill> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}