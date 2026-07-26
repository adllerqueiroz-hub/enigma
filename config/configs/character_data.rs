// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterData {
    pub estimate: String,
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    pub icon: String,
    pub id: i32,
    #[serde(rename = "isCustom")]
    pub is_custom: i32,
    #[serde(rename = "lockText")]
    pub lock_text: i32,
    pub number: i32,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
    pub text: String,
    pub title: String,
    #[serde(rename = "titleEn")]
    pub title_en: String,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "unlockConditine")]
    pub unlock_conditine: String,
    #[serde(rename = "unlockRewards")]
    pub unlock_rewards: String,
}
use std::collections::HashMap;

pub struct CharacterDataTable {
    records: Vec<CharacterData>,
    by_id: HashMap<i32, usize>,
}

impl CharacterDataTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<CharacterData> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&CharacterData> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[CharacterData] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, CharacterData> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}