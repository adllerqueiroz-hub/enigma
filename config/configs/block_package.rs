// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockPackage {
    #[serde(rename = "blockBuildDegree")]
    pub block_build_degree: i32,
    #[serde(rename = "canExchange")]
    pub can_exchange: bool,
    pub desc: String,
    pub free: bool,
    pub icon: String,
    pub id: i32,
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    pub rare: i32,
    #[serde(rename = "rewardIcon")]
    pub reward_icon: String,
    #[serde(rename = "showOnly")]
    pub show_only: bool,
    pub sources: String,
    #[serde(rename = "sourcesType")]
    pub sources_type: String,
    #[serde(rename = "useDesc")]
    pub use_desc: String,
}
use std::collections::HashMap;

pub struct BlockPackageTable {
    records: Vec<BlockPackage>,
    by_id: HashMap<i32, usize>,
}

impl BlockPackageTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<BlockPackage> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&BlockPackage> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[BlockPackage] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, BlockPackage> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}