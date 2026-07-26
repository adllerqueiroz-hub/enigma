// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideStep {
    pub action: String,
    #[serde(rename = "additionCmd")]
    pub addition_cmd: String,
    #[serde(rename = "againSteps")]
    pub again_steps: String,
    pub audio: i32,
    pub delay: f32,
    pub desc: String,
    pub exception: String,
    #[serde(rename = "exceptionDelay")]
    pub exception_delay: i32,
    #[serde(rename = "goPath")]
    pub go_path: String,
    pub id: i32,
    #[serde(rename = "keyStep")]
    pub key_step: i32,
    #[serde(rename = "maskId")]
    pub mask_id: i32,
    #[serde(rename = "notForce")]
    pub not_force: i32,
    #[serde(rename = "portraitPos")]
    pub portrait_pos: i32,
    pub stat: i32,
    #[serde(rename = "stepId")]
    pub step_id: i32,
    #[serde(rename = "storyContent")]
    pub story_content: String,
    #[serde(rename = "tipsContent")]
    pub tips_content: String,
    #[serde(rename = "tipsDir")]
    pub tips_dir: i32,
    #[serde(rename = "tipsHead")]
    pub tips_head: String,
    #[serde(rename = "tipsPos")]
    pub tips_pos: String,
    #[serde(rename = "tipsTalker")]
    pub tips_talker: String,
    #[serde(rename = "touchGOPath")]
    pub touch_gopath: String,
    #[serde(rename = "uiInfo")]
    pub ui_info: String,
    #[serde(rename = "uiOffset")]
    pub ui_offset: String,
}
use std::collections::HashMap;

pub struct GuideStepTable {
    records: Vec<GuideStep>,
    by_id: HashMap<i32, usize>,
}

impl GuideStepTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<GuideStep> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&GuideStep> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[GuideStep] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, GuideStep> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}