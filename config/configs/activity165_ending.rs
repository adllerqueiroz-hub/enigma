// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity165Ending {
    #[serde(rename = "belongStoryId")]
    pub belong_story_id: i32,
    #[serde(rename = "endingId")]
    pub ending_id: i32,
    #[serde(rename = "endingText")]
    pub ending_text: String,
    #[serde(rename = "finalStepId")]
    pub final_step_id: i32,
    pub level: String,
    pub pic: String,
    pub text: String,
}
pub struct Activity165EndingTable {
    records: Vec<Activity165Ending>,
}

impl Activity165EndingTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity165Ending> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity165Ending] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity165Ending> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}