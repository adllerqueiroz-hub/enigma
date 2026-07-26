// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity154 {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "answerId")]
    pub answer_id: i32,
    pub bonus: String,
    pub day: i32,
    #[serde(rename = "puzzleDesc")]
    pub puzzle_desc: String,
    #[serde(rename = "puzzleIcon")]
    pub puzzle_icon: String,
    #[serde(rename = "puzzleId")]
    pub puzzle_id: i32,
    #[serde(rename = "puzzleTitle")]
    pub puzzle_title: String,
}
pub struct Activity154Table {
    records: Vec<Activity154>,
}

impl Activity154Table {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<Activity154> = if let Some(array) = value.as_array() {
            if array.len() >= 2 && array[1].is_array() {
                serde_json::from_value(array[1].clone())?
            } else {
                serde_json::from_value(value)?
            }
        } else {
            serde_json::from_value(value)?
        };

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity154] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity154> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}