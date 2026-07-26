// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentCubeAttr {
    pub absorb: i32,
    pub add_dmg: i32,
    pub atk: i32,
    #[serde(rename = "calculateType")]
    pub calculate_type: i32,
    pub clutch: i32,
    pub cri: i32,
    pub cri_def: i32,
    pub cri_dmg: i32,
    pub def: i32,
    #[serde(rename = "defenseIgnore")]
    pub defense_ignore: i32,
    pub drop_dmg: i32,
    pub heal: i32,
    pub hp: i32,
    pub icon: i32,
    pub id: i32,
    pub level: i32,
    pub mdef: i32,
    #[serde(rename = "normalSkillRate")]
    pub normal_skill_rate: i32,
    pub recri: i32,
    pub revive: i32,
}
use std::collections::HashMap;

pub struct TalentCubeAttrTable {
    records: Vec<TalentCubeAttr>,
    by_id: HashMap<i32, usize>,
}

impl TalentCubeAttrTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<TalentCubeAttr> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&TalentCubeAttr> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[TalentCubeAttr] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TalentCubeAttr> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}