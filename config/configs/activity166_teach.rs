// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166Teach {
    pub desc: String,
    #[serde(rename = "episodeId")]
    pub episode_id: i32,
    #[serde(rename = "firstBonus")]
    pub first_bonus: String,
    pub name: String,
    #[serde(rename = "preTeachId")]
    pub pre_teach_id: i32,
    pub strategy: String,
    #[serde(rename = "teachId")]
    pub teach_id: i32,
}
pub struct Activity166TeachTable {
    records: Vec<Activity166Teach>,
}

impl Activity166TeachTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<Activity166Teach> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity166Teach] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166Teach> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}