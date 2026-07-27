// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomBlockColor {
    #[serde(rename = "blockColor")]
    pub block_color: i32,
    #[serde(rename = "blockId")]
    pub block_id: i32,
    #[serde(rename = "voucherId")]
    pub voucher_id: i32,
}
pub struct RoomBlockColorTable {
    records: Vec<RoomBlockColor>,
}

impl RoomBlockColorTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<RoomBlockColor> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[RoomBlockColor] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomBlockColor> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}