// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity221Map {
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "targetDesc")]
    pub target_desc: String,
    pub targets: String,
    pub time: i32,
    #[serde(rename = "type7Weight")]
    pub type7_weight: String,
}
pub struct Activity221MapTable {
    records: Vec<Activity221Map>,
}

impl Activity221MapTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity221Map> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity221Map] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity221Map> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}