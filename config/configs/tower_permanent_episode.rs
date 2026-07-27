// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerPermanentEpisode {
    #[serde(rename = "episodeIds")]
    pub episode_ids: String,
    #[serde(rename = "firstReward")]
    pub first_reward: String,
    pub index: i32,
    #[serde(rename = "isElite")]
    pub is_elite: i32,
    #[serde(rename = "layerId")]
    pub layer_id: i32,
    #[serde(rename = "preLayerId")]
    pub pre_layer_id: i32,
    #[serde(rename = "spReward")]
    pub sp_reward: String,
    #[serde(rename = "stageId")]
    pub stage_id: i32,
}
pub struct TowerPermanentEpisodeTable {
    records: Vec<TowerPermanentEpisode>,
}

impl TowerPermanentEpisodeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerPermanentEpisode> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TowerPermanentEpisode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerPermanentEpisode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}