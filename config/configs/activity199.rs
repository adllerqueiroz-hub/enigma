// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity199 {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "heroIds")]
    pub hero_ids: String,
}
pub struct Activity199Table {
    records: Vec<Activity199>,
}

impl Activity199Table {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity199> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity199] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity199> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}