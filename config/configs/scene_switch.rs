// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneSwitch {
    #[serde(rename = "defaultUnlock")]
    pub default_unlock: i32,
    #[serde(rename = "eggList")]
    pub egg_list: Option<serde_json::Value>,
    #[serde(rename = "eggSwitchTime")]
    pub egg_switch_time: i32,
    pub icon: String,
    pub id: i32,
    #[serde(rename = "initReportId")]
    pub init_report_id: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "previewIcon")]
    pub preview_icon: String,
    pub previews: Option<serde_json::Value>,
    #[serde(rename = "reportSwitchTime")]
    pub report_switch_time: i32,
    #[serde(rename = "resName")]
    pub res_name: String,
    #[serde(rename = "storyId")]
    pub story_id: i32,
}
use std::collections::HashMap;

pub struct SceneSwitchTable {
    records: Vec<SceneSwitch>,
    by_id: HashMap<i32, usize>,
}

impl SceneSwitchTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<SceneSwitch> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&SceneSwitch> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[SceneSwitch] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SceneSwitch> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}