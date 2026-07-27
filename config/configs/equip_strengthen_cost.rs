// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipStrengthenCost {
    #[serde(rename = "attributeRate")]
    pub attribute_rate: i32,
    pub exp: i32,
    pub level: i32,
    pub rare: i32,
    #[serde(rename = "scoreCost")]
    pub score_cost: i32,
}
pub struct EquipStrengthenCostTable {
    records: Vec<EquipStrengthenCost>,
}

impl EquipStrengthenCostTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<EquipStrengthenCost> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[EquipStrengthenCost] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, EquipStrengthenCost> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}