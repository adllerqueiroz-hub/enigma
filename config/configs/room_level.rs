// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomLevel {
    #[serde(rename = "characterLimit")]
    pub character_limit: i32,
    pub cost: String,
    pub level: i32,
    #[serde(rename = "maxBlockCount")]
    pub max_block_count: i32,
    #[serde(rename = "needBlockCount")]
    pub need_block_count: i32,
    #[serde(rename = "needCost")]
    pub need_cost: String,
    #[serde(rename = "needEpisode")]
    pub need_episode: i32,
}
pub struct RoomLevelTable {
    records: Vec<RoomLevel>,
}

impl RoomLevelTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<RoomLevel> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[RoomLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}