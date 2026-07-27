// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerComposeMod {
    pub career: i32,
    #[serde(rename = "careerChange")]
    pub career_change: String,
    pub desc: String,
    pub icon: String,
    pub id: i32,
    pub image: String,
    #[serde(rename = "isUnlock")]
    pub is_unlock: i32,
    pub level: i32,
    #[serde(rename = "monsterChange")]
    pub monster_change: String,
    pub name: String,
    #[serde(rename = "passiveSkillAdd")]
    pub passive_skill_add: String,
    #[serde(rename = "ruleAdd")]
    pub rule_add: String,
    #[serde(rename = "ruleChange")]
    pub rule_change: String,
    #[serde(rename = "skillChange")]
    pub skill_change: String,
    pub slot: i32,
    #[serde(rename = "slotPart")]
    pub slot_part: String,
    #[serde(rename = "themeId")]
    pub theme_id: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}
use std::collections::HashMap;

pub struct TowerComposeModTable {
    records: Vec<TowerComposeMod>,
    by_id: HashMap<i32, usize>,
}

impl TowerComposeModTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerComposeMod> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&TowerComposeMod> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[TowerComposeMod] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerComposeMod> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}