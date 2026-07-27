// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerComposeSupport {
    #[serde(rename = "activeSkills")]
    pub active_skills: String,
    #[serde(rename = "activeType")]
    pub active_type: i32,
    #[serde(rename = "coldTime")]
    pub cold_time: i32,
    pub desc: String,
    #[serde(rename = "extraRule")]
    pub extra_rule: String,
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    #[serde(rename = "heroTag")]
    pub hero_tag: i32,
    pub id: i32,
    pub lv: i32,
    #[serde(rename = "passiveSkills")]
    pub passive_skills: String,
    #[serde(rename = "resInitVal")]
    pub res_init_val: i32,
    #[serde(rename = "resMaxVal")]
    pub res_max_val: i32,
    #[serde(rename = "themeId")]
    pub theme_id: i32,
}
use std::collections::HashMap;

pub struct TowerComposeSupportTable {
    records: Vec<TowerComposeSupport>,
    by_id: HashMap<i32, usize>,
}

impl TowerComposeSupportTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerComposeSupport> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&TowerComposeSupport> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[TowerComposeSupport] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerComposeSupport> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}