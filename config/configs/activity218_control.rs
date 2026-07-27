// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity218Control {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "drawPoint")]
    pub draw_point: i32,
    #[serde(rename = "losePoint")]
    pub lose_point: i32,
    pub times: i32,
    #[serde(rename = "winPoint")]
    pub win_point: i32,
}
pub struct Activity218ControlTable {
    records: Vec<Activity218Control>,
}

impl Activity218ControlTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity218Control> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity218Control] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity218Control> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}