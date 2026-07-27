// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroStory {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub activity_pic: String,
    pub bonus: String,
    #[serde(rename = "cgBg")]
    pub cg_bg: String,
    #[serde(rename = "cgPos")]
    pub cg_pos: String,
    #[serde(rename = "cgScale")]
    pub cg_scale: String,
    #[serde(rename = "cgUnlockEpisodeId")]
    pub cg_unlock_episode_id: i32,
    #[serde(rename = "cgUnlockStoryId")]
    pub cg_unlock_story_id: i32,
    #[serde(rename = "challengeBonus")]
    pub challenge_bonus: String,
    #[serde(rename = "chapterId")]
    pub chapter_id: i32,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "heroName")]
    pub hero_name: String,
    pub id: i32,
    pub main_pic: String,
    #[serde(rename = "mainviewName")]
    pub mainview_name: String,
    pub monster_pic: String,
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    pub order: i32,
    #[serde(rename = "permanentUnlock")]
    pub permanent_unlock: String,
    pub photo: String,
    #[serde(rename = "preStoryId")]
    pub pre_story_id: i32,
    #[serde(rename = "queryVersion")]
    pub query_version: String,
    pub signature: String,
    pub unlock: String,
}
use std::collections::HashMap;

pub struct HeroStoryTable {
    records: Vec<HeroStory>,
    by_id: HashMap<i32, usize>,
}

impl HeroStoryTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<HeroStory> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&HeroStory> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[HeroStory] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, HeroStory> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}