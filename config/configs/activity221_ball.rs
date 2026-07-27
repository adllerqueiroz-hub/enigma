// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity221Ball {
    pub action: String,
    #[serde(rename = "ballType")]
    pub ball_type: i32,
    pub image: String,
    #[serde(rename = "skillIds")]
    pub skill_ids: String,
}
pub struct Activity221BallTable {
    records: Vec<Activity221Ball>,
}

impl Activity221BallTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity221Ball> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity221Ball] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity221Ball> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}