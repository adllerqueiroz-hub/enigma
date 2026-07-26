// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopostPasswordPaper {
    #[serde(rename = "allNum")]
    pub all_num: i32,
    #[serde(rename = "diskIcon")]
    pub disk_icon: String,
    #[serde(rename = "diskText")]
    pub disk_text: String,
    pub id: i32,
    pub item: String,
    pub order: Option<serde_json::Value>,
    #[serde(rename = "versionId")]
    pub version_id: i32,
    #[serde(rename = "viewType")]
    pub view_type: i32,
}
use std::collections::HashMap;

pub struct CopostPasswordPaperTable {
    records: Vec<CopostPasswordPaper>,
    by_id: HashMap<i32, usize>,
}

impl CopostPasswordPaperTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<CopostPasswordPaper> = if let Some(array) = value.as_array() {
            if array.len() >= 2 && array[1].is_array() {
                serde_json::from_value(array[1].clone())?
            } else {
                serde_json::from_value(value)?
            }
        } else {
            serde_json::from_value(value)?
        };

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
    pub fn get(&self, id: i32) -> Option<&CopostPasswordPaper> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[CopostPasswordPaper] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CopostPasswordPaper> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}