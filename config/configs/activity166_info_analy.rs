// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166InfoAnaly {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    pub consume: i32,
    pub content: String,
    #[serde(rename = "infoId")]
    pub info_id: i32,
    pub stage: i32,
}
pub struct Activity166InfoAnalyTable {
    records: Vec<Activity166InfoAnaly>,
}

impl Activity166InfoAnalyTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity166InfoAnaly> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity166InfoAnaly] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166InfoAnaly> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}