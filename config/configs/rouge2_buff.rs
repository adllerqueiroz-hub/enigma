// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rouge2Buff {
    #[serde(rename = "attributeTag")]
    pub attribute_tag: String,
    #[serde(rename = "battleTag")]
    pub battle_tag: String,
    pub career: String,
    pub desc: String,
    #[serde(rename = "descSimply")]
    pub desc_simply: String,
    #[serde(rename = "descUpdate")]
    pub desc_update: String,
    pub icon: String,
    pub id: i32,
    #[serde(rename = "isAttrBuff")]
    pub is_attr_buff: i32,
    #[serde(rename = "isHide")]
    pub is_hide: i32,
    #[serde(rename = "isOff")]
    pub is_off: i32,
    pub name: String,
    #[serde(rename = "outUnlock")]
    pub out_unlock: String,
    #[serde(rename = "outUnlockDesc")]
    pub out_unlock_desc: String,
    #[serde(rename = "passiveSkillId")]
    pub passive_skill_id: String,
    pub rare: i32,
    #[serde(rename = "sortId")]
    pub sort_id: i32,
    pub system: i32,
    pub tag: String,
    pub unlock: String,
    #[serde(rename = "updateId")]
    pub update_id: i32,
}
use std::collections::HashMap;

pub struct Rouge2BuffTable {
    records: Vec<Rouge2Buff>,
    by_id: HashMap<i32, usize>,
}

impl Rouge2BuffTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<Rouge2Buff> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Rouge2Buff> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Rouge2Buff] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rouge2Buff> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}