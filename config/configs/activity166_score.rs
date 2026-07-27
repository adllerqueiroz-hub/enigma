// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166Score {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub level: i32,
    #[serde(rename = "needScore")]
    pub need_score: i32,
    pub star: i32,
}
pub struct Activity166ScoreTable {
    records: Vec<Activity166Score>,
}

impl Activity166ScoreTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity166Score> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity166Score] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166Score> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}