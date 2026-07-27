// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroStoryPlot {
    #[serde(rename = "addControl")]
    pub add_control: String,
    #[serde(rename = "controlDelay")]
    pub control_delay: String,
    #[serde(rename = "controlParam")]
    pub control_param: String,
    pub desc: String,
    pub id: i32,
    pub level: String,
    pub name: String,
    pub param: String,
    pub pause: i32,
    pub storygroup: i32,
    #[serde(rename = "type")]
    pub r#type: String,
}
use std::collections::HashMap;

pub struct HeroStoryPlotTable {
    records: Vec<HeroStoryPlot>,
    by_id: HashMap<i32, usize>,
}

impl HeroStoryPlotTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<HeroStoryPlot> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&HeroStoryPlot> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[HeroStoryPlot] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, HeroStoryPlot> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}