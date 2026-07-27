// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerComposeTheme {
    pub career: String,
    #[serde(rename = "heroIds")]
    pub hero_ids: String,
    pub id: i32,
    #[serde(rename = "initEnv")]
    pub init_env: i32,
    #[serde(rename = "isOnline")]
    pub is_online: i32,
    #[serde(rename = "modNum")]
    pub mod_num: String,
    #[serde(rename = "modOffset")]
    pub mod_offset: String,
    #[serde(rename = "monsterGroupId")]
    pub monster_group_id: i32,
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "orderLayer")]
    pub order_layer: String,
    #[serde(rename = "pointIcon")]
    pub point_icon: String,
    #[serde(rename = "spineOffset")]
    pub spine_offset: String,
    #[serde(rename = "themeDesc")]
    pub theme_desc: String,
    #[serde(rename = "themeIcon")]
    pub theme_icon: String,
}
use std::collections::HashMap;

pub struct TowerComposeThemeTable {
    records: Vec<TowerComposeTheme>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl TowerComposeThemeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerComposeTheme> = crate::load_rows(path)?;

        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();

        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.monster_group_id).or_default().push(idx);
        }

        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&TowerComposeTheme> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &'_ TowerComposeTheme> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|idxs| idxs.iter())
            .map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[TowerComposeTheme] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerComposeTheme> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}