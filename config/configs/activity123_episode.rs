// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity123Episode {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub desc: String,
    #[serde(rename = "displayMark")]
    pub display_mark: i32,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "gottenEquip")]
    pub gotten_equip: String,
    pub layer: i32,
    #[serde(rename = "layerName")]
    pub layer_name: String,
    #[serde(rename = "layerPicture")]
    pub layer_picture: String,
    pub level: i32,
    #[serde(rename = "recommendInfo")]
    pub recommend_info: String,
    pub stage: i32,
    #[serde(rename = "stagePicture")]
    pub stage_picture: String,
    #[serde(rename = "unlockEquipIndex")]
    pub unlock_equip_index: String,
    #[serde(rename = "usableEquip")]
    pub usable_equip: String,
}
pub struct Activity123EpisodeTable {
    records: Vec<Activity123Episode>,
}

impl Activity123EpisodeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity123Episode> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity123Episode] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity123Episode> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}