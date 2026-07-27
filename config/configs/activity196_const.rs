// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity196Const {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub time: String,
}
pub struct Activity196ConstTable {
    records: Vec<Activity196Const>,
}

impl Activity196ConstTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity196Const> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity196Const] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity196Const> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}