// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingBonus {
    pub bonus: i32,
    #[serde(rename = "buildDegree")]
    pub build_degree: i32,
    #[serde(rename = "characterLimitAdd")]
    pub character_limit_add: i32,
}
pub struct BuildingBonusTable {
    records: Vec<BuildingBonus>,
}

impl BuildingBonusTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<BuildingBonus> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[BuildingBonus] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, BuildingBonus> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}