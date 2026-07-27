// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroStoryPlotGroup {
    pub branch: i32,
    pub date: String,
    pub id: i32,
    #[serde(rename = "isEnd")]
    pub is_end: String,
    pub place: String,
    #[serde(rename = "preId")]
    pub pre_id: i32,
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "storyId")]
    pub story_id: i32,
    #[serde(rename = "storyName")]
    pub story_name: String,
    #[serde(rename = "storyNameEn")]
    pub story_name_en: String,
    #[serde(rename = "storyPic")]
    pub story_pic: String,
    pub time: f32,
    pub weather: i32,
}
use std::collections::HashMap;

pub struct HeroStoryPlotGroupTable {
    records: Vec<HeroStoryPlotGroup>,
    by_id: HashMap<i32, usize>,
}

impl HeroStoryPlotGroupTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<HeroStoryPlotGroup> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&HeroStoryPlotGroup> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[HeroStoryPlotGroup] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, HeroStoryPlotGroup> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}