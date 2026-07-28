// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterPointReward {
    #[serde(rename = "chapterId")]
    pub chapter_id: i32,
    pub display: i32,
    pub id: i32,
    pub reward: String,
    #[serde(rename = "rewardPointNum")]
    pub reward_point_num: i32,
    #[serde(rename = "unlockChapter")]
    pub unlock_chapter: i32,
}
use std::collections::HashMap;

pub struct ChapterPointRewardTable {
    records: Vec<ChapterPointReward>,
    by_id: HashMap<i32, usize>,
}

impl ChapterPointRewardTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<ChapterPointReward> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&ChapterPointReward> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[ChapterPointReward] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, ChapterPointReward> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}