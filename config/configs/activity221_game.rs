// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity221Game {
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "maxBuff")]
    pub max_buff: i32,
    #[serde(rename = "targetDesc")]
    pub target_desc: String,
    pub targets: String,
    pub time: i32,
    #[serde(rename = "type7Num")]
    pub type7_num: String,
    #[serde(rename = "type7Weight")]
    pub type7_weight: String,
    #[serde(rename = "useSkill")]
    pub use_skill: String,
}
pub struct Activity221GameTable {
    records: Vec<Activity221Game>,
}

impl Activity221GameTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity221Game> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity221Game] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity221Game> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}