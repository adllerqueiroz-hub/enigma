// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalRole {
    #[serde(rename = "chessPic")]
    pub chess_pic: String,
    pub conditions: String,
    #[serde(rename = "conditionsDesc")]
    pub conditions_desc: String,
    pub desc: String,
    #[serde(rename = "dispositionType")]
    pub disposition_type: i32,
    pub head: String,
    pub id: i32,
    #[serde(rename = "initDisposition")]
    pub init_disposition: String,
    #[serde(rename = "initTalentIds")]
    pub init_talent_ids: String,
    pub isonline: i32,
    #[serde(rename = "mapHead")]
    pub map_head: String,
    #[serde(rename = "moveHead")]
    pub move_head: String,
    pub name: String,
    pub pic: String,
    pub resource: String,
    pub skill: i32,
    #[serde(rename = "talentName")]
    pub talent_name: String,
    #[serde(rename = "techIconType")]
    pub tech_icon_type: i32,
    #[serde(rename = "techSpriteId")]
    pub tech_sprite_id: i32,
}
use std::collections::HashMap;

pub struct SurvivalRoleTable {
    records: Vec<SurvivalRole>,
    by_id: HashMap<i32, usize>,
}

impl SurvivalRoleTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<SurvivalRole> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SurvivalRole> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[SurvivalRole] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SurvivalRole> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}