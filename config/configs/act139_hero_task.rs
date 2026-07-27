// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act139HeroTask {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub desc: String,
    #[serde(rename = "heroIcon")]
    pub hero_icon: String,
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    #[serde(rename = "heroTabIcon")]
    pub hero_tab_icon: String,
    pub id: i32,
    #[serde(rename = "preEpisodeId")]
    pub pre_episode_id: i32,
    pub reward: String,
    pub title: String,
    #[serde(rename = "toastId")]
    pub toast_id: i32,
}
use std::collections::HashMap;

pub struct Act139HeroTaskTable {
    records: Vec<Act139HeroTask>,
    by_id: HashMap<i32, usize>,
}

impl Act139HeroTaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Act139HeroTask> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Act139HeroTask> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Act139HeroTask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Act139HeroTask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}