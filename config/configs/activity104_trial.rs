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
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<Activity104Trial> = if let Some(array) = value.as_array() {
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