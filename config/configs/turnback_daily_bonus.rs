// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnbackDailyBonus {
    pub bonus: String,
    pub day: i32,
    #[serde(rename = "turnbackId")]
    pub turnback_id: i32,
}
pub struct TurnbackDailyBonusTable {
    records: Vec<TurnbackDailyBonus>,
}

impl TurnbackDailyBonusTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TurnbackDailyBonus> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TurnbackDailyBonus] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TurnbackDailyBonus> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}