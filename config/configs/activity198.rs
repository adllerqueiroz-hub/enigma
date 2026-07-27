// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity198 {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    pub num: i32,
    #[serde(rename = "skinIds")]
    pub skin_ids: String,
}
pub struct Activity198Table {
    records: Vec<Activity198>,
}

impl Activity198Table {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity198> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity198] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity198> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}