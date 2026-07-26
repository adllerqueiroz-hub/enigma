// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdysseyTalent {
    #[serde(rename = "addRule")]
    pub add_rule: String,
    #[serde(rename = "addSkill")]
    pub add_skill: String,
    pub consume: i32,
    pub icon: String,
    pub level: i32,
    #[serde(rename = "nodeDesc")]
    pub node_desc: String,
    #[serde(rename = "nodeId")]
    pub node_id: i32,
    #[serde(rename = "nodeName")]
    pub node_name: String,
    pub position: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "unlockCondition")]
    pub unlock_condition: String,
}
pub struct OdysseyTalentTable {
    records: Vec<OdysseyTalent>,
}

impl OdysseyTalentTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<OdysseyTalent> = if let Some(array) = value.as_array() {
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
    pub fn all(&self) -> &[OdysseyTalent] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, OdysseyTalent> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}