// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneUi {
    #[serde(rename = "defaultUnlock")]
    pub default_unlock: i32,
    pub icon: String,
    pub id: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "previewIcon")]
    pub preview_icon: String,
}
use std::collections::HashMap;

pub struct SceneUiTable {
    records: Vec<SceneUi>,
    by_id: HashMap<i32, usize>,
}

impl SceneUiTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<SceneUi> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&SceneUi> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[SceneUi] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SceneUi> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}