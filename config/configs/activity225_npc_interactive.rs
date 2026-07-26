// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity225NpcInteractive {
    #[serde(rename = "interactiveId")]
    pub interactive_id: i32,
    #[serde(rename = "interactivePng")]
    pub interactive_png: String,
    #[serde(rename = "interactiveTxt")]
    pub interactive_txt: String,
    #[serde(rename = "interactiveType")]
    pub interactive_type: i32,
}
pub struct Activity225NpcInteractiveTable {
    records: Vec<Activity225NpcInteractive>,
}

impl Activity225NpcInteractiveTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<Activity225NpcInteractive> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[Activity225NpcInteractive] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity225NpcInteractive> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}