// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity218Dailydifficultly {
    pub day: i32,
    pub difficulty: String,
}
pub struct Activity218DailydifficultlyTable {
    records: Vec<Activity218Dailydifficultly>,
}

impl Activity218DailydifficultlyTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity218Dailydifficultly> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity218Dailydifficultly] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity218Dailydifficultly> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}