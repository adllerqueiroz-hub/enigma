// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity165Story {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "firstStepId")]
    pub first_step_id: i32,
    #[serde(rename = "firstUnlockElementCd1")]
    pub first_unlock_element_cd1: i32,
    #[serde(rename = "firstUnlockElementCd2")]
    pub first_unlock_element_cd2: i32,
    pub name: String,
    pub pic: String,
    #[serde(rename = "preElementId1")]
    pub pre_element_id1: i32,
    #[serde(rename = "preElementId2")]
    pub pre_element_id2: i32,
    #[serde(rename = "storyId")]
    pub story_id: i32,
    #[serde(rename = "unlockElementIds1")]
    pub unlock_element_ids1: String,
    #[serde(rename = "unlockElementIds2")]
    pub unlock_element_ids2: String,
}
pub struct Activity165StoryTable {
    records: Vec<Activity165Story>,
}

impl Activity165StoryTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity165Story> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity165Story] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity165Story> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}