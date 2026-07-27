// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity104Trial {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "equipId")]
    pub equip_id: i32,
    #[serde(rename = "firstPassEquipIds")]
    pub first_pass_equip_ids: String,
    pub layer: i32,
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "unlockLayer")]
    pub unlock_layer: i32,
}
pub struct Activity104TrialTable {
    records: Vec<Activity104Trial>,
}

impl Activity104TrialTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity104Trial> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity104Trial] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity104Trial> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}