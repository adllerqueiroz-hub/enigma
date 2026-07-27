// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipStrengthen {
    pub atk: i32,
    pub def: i32,
    pub hp: i32,
    pub mdef: i32,
    #[serde(rename = "strengthType")]
    pub strength_type: i32,
}
pub struct EquipStrengthenTable {
    records: Vec<EquipStrengthen>,
}

impl EquipStrengthenTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<EquipStrengthen> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[EquipStrengthen] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, EquipStrengthen> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}