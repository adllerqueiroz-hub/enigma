// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillBuffSelfRefresh {
    #[serde(rename = "buffId")]
    pub buff_id: i32,
    #[serde(rename = "refreshMode")]
    pub refresh_mode: String,
}
pub struct SkillBuffSelfRefreshTable {
    records: Vec<SkillBuffSelfRefresh>,
}

impl SkillBuffSelfRefreshTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<SkillBuffSelfRefresh> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[SkillBuffSelfRefresh] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SkillBuffSelfRefresh> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}