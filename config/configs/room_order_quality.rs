// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomOrderQuality {
    #[serde(rename = "goodsWeight")]
    pub goods_weight: String,
    pub price: String,
    pub quality: i32,
    #[serde(rename = "typeCount")]
    pub type_count: String,
}
pub struct RoomOrderQualityTable {
    records: Vec<RoomOrderQuality>,
}

impl RoomOrderQualityTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<RoomOrderQuality> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[RoomOrderQuality] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomOrderQuality> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}