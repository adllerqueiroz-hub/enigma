// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity197Pool {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    pub index: i32,
    #[serde(rename = "poolId")]
    pub pool_id: i32,
}
pub struct Activity197PoolTable {
    records: Vec<Activity197Pool>,
}

impl Activity197PoolTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity197Pool> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity197Pool] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity197Pool> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}