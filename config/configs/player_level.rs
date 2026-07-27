// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerLevel {
    #[serde(rename = "addBuyRecoverPower")]
    pub add_buy_recover_power: i32,
    #[serde(rename = "addUpRecoverPower")]
    pub add_up_recover_power: i32,
    pub bonus: i32,
    pub exp: i32,
    pub level: i32,
    #[serde(rename = "maxAutoRecoverPower")]
    pub max_auto_recover_power: i32,
}
pub struct PlayerLevelTable {
    records: Vec<PlayerLevel>,
}

impl PlayerLevelTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<PlayerLevel> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[PlayerLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, PlayerLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}