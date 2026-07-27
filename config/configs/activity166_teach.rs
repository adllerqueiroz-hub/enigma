// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166Teach {
    pub desc: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "firstBonus")]
    pub first_bonus: String,
    pub name: String,
    #[serde(rename = "preTeachId")]
    pub pre_teach_id: i32,
    pub strategy: String,
    #[serde(rename = "teachId")]
    pub teach_id: i32,
}
pub struct Activity166TeachTable {
    records: Vec<Activity166Teach>,
}

impl Activity166TeachTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity166Teach> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity166Teach] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166Teach> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}