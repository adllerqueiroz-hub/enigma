// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummonPoolPackage {
    #[serde(rename = "className")]
    pub class_name: String,
    pub id: i32,
    pub order: i32,
    #[serde(rename = "packageEffect")]
    pub package_effect: i32,
    #[serde(rename = "packageRecommend")]
    pub package_recommend: String,
    #[serde(rename = "packageRecommendSwitch")]
    pub package_recommend_switch: bool,
    #[serde(rename = "posOffset")]
    pub pos_offset: String,
    #[serde(rename = "showLimit")]
    pub show_limit: String,
}
use std::collections::HashMap;

pub struct SummonPoolPackageTable {
    records: Vec<SummonPoolPackage>,
    by_id: HashMap<i32, usize>,
}

impl SummonPoolPackageTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<SummonPoolPackage> = if let Some(array) = value.as_array() {
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
    pub fn get(&self, id: i32) -> Option<&SummonPoolPackage> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[SummonPoolPackage] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SummonPoolPackage> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}