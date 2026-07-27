// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufactureItem {
    #[serde(rename = "batchIcon")]
    pub batch_icon: String,
    #[serde(rename = "batchName")]
    pub batch_name: String,
    #[serde(rename = "criProductionCount")]
    pub cri_production_count: i32,
    #[serde(rename = "criProductionId")]
    pub cri_production_id: i32,
    pub id: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "needMat")]
    pub need_mat: String,
    #[serde(rename = "needTime")]
    pub need_time: i32,
    #[serde(rename = "orderPrice")]
    pub order_price: i32,
    #[serde(rename = "showInAdvancedOrder")]
    pub show_in_advanced_order: bool,
    #[serde(rename = "unitCount")]
    pub unit_count: i32,
    #[serde(rename = "wholesalePrice")]
    pub wholesale_price: i32,
}
use std::collections::HashMap;

pub struct ManufactureItemTable {
    records: Vec<ManufactureItem>,
    by_id: HashMap<i32, usize>,
}

impl ManufactureItemTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<ManufactureItem> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&ManufactureItem> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[ManufactureItem] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, ManufactureItem> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}