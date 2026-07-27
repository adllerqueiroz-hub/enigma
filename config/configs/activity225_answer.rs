// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity225Answer {
    #[serde(rename = "answerComment")]
    pub answer_comment: String,
    #[serde(rename = "answerId")]
    pub answer_id: i32,
    #[serde(rename = "answerTxt")]
    pub answer_txt: String,
}
pub struct Activity225AnswerTable {
    records: Vec<Activity225Answer>,
}

impl Activity225AnswerTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity225Answer> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity225Answer] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity225Answer> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}