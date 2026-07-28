// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Udimo {
    #[serde(rename = "colliderOffset")]
    pub collider_offset: String,
    #[serde(rename = "colliderSize")]
    pub collider_size: String,
    #[serde(rename = "defaultUse")]
    pub default_use: i32,
    #[serde(rename = "emojiPos")]
    pub emoji_pos: Vec<f32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    pub id: i32,
    #[serde(rename = "imgParam")]
    pub img_param: Vec<i32>,
    #[serde(rename = "isDefault")]
    pub is_default: i32,
    pub name: String,
    #[serde(rename = "orderLayer")]
    pub order_layer: i32,
    #[serde(rename = "outlineRes")]
    pub outline_res: String,
    pub resource: String,
    #[serde(rename = "resourceParam")]
    pub resource_param: f32,
    #[serde(rename = "spineParam")]
    pub spine_param: Vec<f32>,
}
use std::collections::HashMap;

pub struct UdimoTable {
    records: Vec<Udimo>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl UdimoTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Udimo> = crate::load_rows(path)?;

        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();

        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.group_id).or_default().push(idx);
        }

        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Udimo> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &'_ Udimo> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|idxs| idxs.iter())
            .map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Udimo] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Udimo> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}