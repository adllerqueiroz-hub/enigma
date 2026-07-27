// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipBreakCost {
    #[serde(rename = "breakLevel")]
    pub break_level: i32,
    pub cost: String,
    pub level: i32,
    pub rare: i32,
    #[serde(rename = "scoreCost")]
    pub score_cost: i32,
}
pub struct EquipBreakCostTable {
    records: Vec<EquipBreakCost>,
}

impl EquipBreakCostTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<EquipBreakCost> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[EquipBreakCost] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, EquipBreakCost> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}