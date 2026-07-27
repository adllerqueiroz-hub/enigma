// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SameCurrencyExchange {
    #[serde(rename = "boxPath")]
    pub box_path: String,
    #[serde(rename = "currencyId")]
    pub currency_id: i32,
    pub desc: String,
    pub image: String,
    #[serde(rename = "storeEntranceId")]
    pub store_entrance_id: String,
    #[serde(rename = "versionId")]
    pub version_id: i32,
}
pub struct SameCurrencyExchangeTable {
    records: Vec<SameCurrencyExchange>,
}

impl SameCurrencyExchangeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<SameCurrencyExchange> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[SameCurrencyExchange] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SameCurrencyExchange> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}