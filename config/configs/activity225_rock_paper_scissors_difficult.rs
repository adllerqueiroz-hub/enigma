// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity225RockPaperScissorsDifficult {
    pub day: i32,
    pub difficulty: String,
}
pub struct Activity225RockPaperScissorsDifficultTable {
    records: Vec<Activity225RockPaperScissorsDifficult>,
}

impl Activity225RockPaperScissorsDifficultTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity225RockPaperScissorsDifficult> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity225RockPaperScissorsDifficult] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity225RockPaperScissorsDifficult> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}