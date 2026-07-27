// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity217Control {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "dailyLimit")]
    pub daily_limit: i32,
    pub limit: i32,
    pub magnification: i32,
    pub showtype: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}
pub struct Activity217ControlTable {
    records: Vec<Activity217Control>,
}

impl Activity217ControlTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity217Control> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity217Control] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity217Control> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}