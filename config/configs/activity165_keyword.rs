// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity165Keyword {
    #[serde(rename = "belongStoryId")]
    pub belong_story_id: i32,
    #[serde(rename = "keywordId")]
    pub keyword_id: i32,
    pub pic: String,
    pub text: String,
}
pub struct Activity165KeywordTable {
    records: Vec<Activity165Keyword>,
}

impl Activity165KeywordTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity165Keyword> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity165Keyword] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity165Keyword> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}