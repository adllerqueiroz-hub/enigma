// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rouge2Career {
    #[serde(rename = "activeSkills")]
    pub active_skills: String,
    #[serde(rename = "attrMapBg")]
    pub attr_map_bg: String,
    #[serde(rename = "audioId")]
    pub audio_id: i32,
    #[serde(rename = "bagEntranceIcon")]
    pub bag_entrance_icon: String,
    #[serde(rename = "careerDesc")]
    pub career_desc: String,
    #[serde(rename = "heroArmIcon")]
    pub hero_arm_icon: String,
    pub icon: String,
    pub id: i32,
    #[serde(rename = "initialAttribute")]
    pub initial_attribute: String,
    #[serde(rename = "initialColletions")]
    pub initial_colletions: String,
    #[serde(rename = "initialEffects")]
    pub initial_effects: String,
    #[serde(rename = "initialRevivalCoin")]
    pub initial_revival_coin: i32,
    #[serde(rename = "isDifficult")]
    pub is_difficult: i32,
    #[serde(rename = "mpInitial")]
    pub mp_initial: i32,
    #[serde(rename = "mpMax")]
    pub mp_max: i32,
    pub name: String,
    #[serde(rename = "nameColor")]
    pub name_color: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "passiveSkillBrief")]
    pub passive_skill_brief: String,
    #[serde(rename = "passiveSkills")]
    pub passive_skills: String,
    #[serde(rename = "recommendAttribute")]
    pub recommend_attribute: String,
    #[serde(rename = "recommendTeam")]
    pub recommend_team: String,
    #[serde(rename = "sortAttribute")]
    pub sort_attribute: String,
    pub unlock: String,
    #[serde(rename = "unlockTime")]
    pub unlock_time: String,
}
use std::collections::HashMap;

pub struct Rouge2CareerTable {
    records: Vec<Rouge2Career>,
    by_id: HashMap<i32, usize>,
}

impl Rouge2CareerTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<Rouge2Career> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&Rouge2Career> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Rouge2Career] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rouge2Career> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}