// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity240Backdate {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub cost: String,
}
pub struct Activity240BackdateTable {
    records: Vec<Activity240Backdate>,
}

impl Activity240BackdateTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity240Backdate> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity240Backdate] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity240Backdate> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}