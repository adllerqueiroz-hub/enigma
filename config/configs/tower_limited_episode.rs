// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerLimitedEpisode {
    pub difficulty: i32,
    pub entrance: i32,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "layerId")]
    pub layer_id: i32,
    pub season: i32,
}
pub struct TowerLimitedEpisodeTable {
    records: Vec<TowerLimitedEpisode>,
}

impl TowerLimitedEpisodeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerLimitedEpisode> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TowerLimitedEpisode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerLimitedEpisode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}