// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rouge2Relics {
    #[serde(rename = "attrUpdate")]
    pub attr_update: String,
    #[serde(rename = "attributeTag")]
    pub attribute_tag: String,
    pub career: String,
    pub condition1: String,
    pub condition10: String,
    pub condition11: String,
    pub condition12: String,
    pub condition13: String,
    pub condition14: String,
    pub condition15: String,
    pub condition16: String,
    pub condition17: String,
    pub condition18: String,
    pub condition19: String,
    pub condition2: String,
    pub condition20: String,
    pub condition21: String,
    pub condition22: String,
    pub condition23: String,
    pub condition24: String,
    pub condition25: String,
    pub condition3: String,
    pub condition4: String,
    pub condition5: String,
    pub condition6: String,
    pub condition7: String,
    pub condition8: String,
    pub condition9: String,
    pub desc: String,
    #[serde(rename = "descSimply")]
    pub desc_simply: String,
    #[serde(rename = "descUpdate")]
    pub desc_update: String,
    pub effect1: String,
    pub effect10: String,
    pub effect11: String,
    pub effect12: String,
    pub effect13: String,
    pub effect14: String,
    pub effect15: String,
    pub effect16: String,
    pub effect17: String,
    pub effect18: String,
    pub effect19: String,
    pub effect2: String,
    pub effect20: String,
    pub effect21: String,
    pub effect22: String,
    pub effect23: String,
    pub effect24: String,
    pub effect25: String,
    pub effect3: String,
    pub effect4: String,
    pub effect5: String,
    pub effect6: String,
    pub effect7: String,
    pub effect8: String,
    pub effect9: String,
    pub icon: String,
    pub id: i32,
    pub invisible: i32,
    #[serde(rename = "isDisplay")]
    pub is_display: i32,
    #[serde(rename = "isHide")]
    pub is_hide: i32,
    #[serde(rename = "isOff")]
    pub is_off: i32,
    pub name: String,
    #[serde(rename = "narrativeDesc")]
    pub narrative_desc: String,
    #[serde(rename = "outUnlock")]
    pub out_unlock: String,
    #[serde(rename = "outUnlockDesc")]
    pub out_unlock_desc: String,
    pub overlay: String,
    pub rare: i32,
    #[serde(rename = "sortId")]
    pub sort_id: i32,
    pub tag: String,
    pub unlock: String,
    #[serde(rename = "unlockConditionDesc")]
    pub unlock_condition_desc: String,
    #[serde(rename = "unlockEffectDesc")]
    pub unlock_effect_desc: String,
    #[serde(rename = "updateId")]
    pub update_id: i32,
}
use std::collections::HashMap;

pub struct Rouge2RelicsTable {
    records: Vec<Rouge2Relics>,
    by_id: HashMap<i32, usize>,
}

impl Rouge2RelicsTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<Rouge2Relics> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Rouge2Relics> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Rouge2Relics] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rouge2Relics> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}