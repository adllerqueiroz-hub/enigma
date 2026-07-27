// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity225Question {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "answerIds")]
    pub answer_ids: String,
    pub id: i32,
    #[serde(rename = "npcId")]
    pub npc_id: i32,
    pub question: String,
}
use std::collections::HashMap;

pub struct Activity225QuestionTable {
    records: Vec<Activity225Question>,
    by_id: HashMap<i32, usize>,
}

impl Activity225QuestionTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity225Question> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Activity225Question> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Activity225Question] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity225Question> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}