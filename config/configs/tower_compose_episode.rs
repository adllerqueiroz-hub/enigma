// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerComposeEpisode {
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "layerId")]
    pub layer_id: i32,
    pub name: String,
    #[serde(rename = "nextLayerId")]
    pub next_layer_id: i32,
    pub plane: i32,
    #[serde(rename = "stageId")]
    pub stage_id: i32,
    #[serde(rename = "themeId")]
    pub theme_id: i32,
    pub unlock: String,
    #[serde(rename = "unlockModIds")]
    pub unlock_mod_ids: String,
}
pub struct TowerComposeEpisodeTable {
    records: Vec<TowerComposeEpisode>,
}

impl TowerComposeEpisodeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<TowerComposeEpisode> = if let Some(array) = value.as_array() {
            if array.len() >= 2 && array[1].is_array() {
                serde_json::from_value(array[1].clone())?
            } else {
                serde_json::from_value(value)?
            }
        } else {
            serde_json::from_value(value)?
        };

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TowerComposeEpisode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerComposeEpisode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}