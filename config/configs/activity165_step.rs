// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity165Step {
    #[serde(rename = "answersKeywordIds")]
    pub answers_keyword_ids: String,
    #[serde(rename = "belongStoryId")]
    pub belong_story_id: i32,
    #[serde(rename = "nextStepConditionIds")]
    pub next_step_condition_ids: String,
    #[serde(rename = "optionalKeywordIds")]
    pub optional_keyword_ids: String,
    pub pic: String,
    #[serde(rename = "stepId")]
    pub step_id: i32,
    pub text: String,
}
pub struct Activity165StepTable {
    records: Vec<Activity165Step>,
}

impl Activity165StepTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity165Step> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity165Step] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity165Step> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}