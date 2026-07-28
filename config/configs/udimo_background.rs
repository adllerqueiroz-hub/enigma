// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdimoBackground {
    #[serde(rename = "airPoints")]
    pub air_points: String,
    pub bgm: i32,
    #[serde(rename = "cameraMoveRange")]
    pub camera_move_range: String,
    #[serde(rename = "defaultUse")]
    pub default_use: i32,
    #[serde(rename = "defaultWeather")]
    pub default_weather: i32,
    pub id: i32,
    pub img: String,
    #[serde(rename = "isDefault")]
    pub is_default: i32,
    pub name: String,
    #[serde(rename = "sceneLevel")]
    pub scene_level: i32,
}
use std::collections::HashMap;

pub struct UdimoBackgroundTable {
    records: Vec<UdimoBackground>,
    by_id: HashMap<i32, usize>,
}

impl UdimoBackgroundTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<UdimoBackground> = crate::load_rows(path)?;

        let mut by_id = HashMap::with_capacity(records.len());

        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }

        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&UdimoBackground> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[UdimoBackground] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, UdimoBackground> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}