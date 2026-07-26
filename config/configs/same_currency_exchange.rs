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
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<SameCurrencyExchange> = if let Some(array) = value.as_array() {
            if array.len() >= 2 && array[1].is_array() {
                serde_json::from_value(array[1].clone())?
            } else {
                serde_json::from_value(value)?
            }
        } else {
            serde_json::from_value(value)?
        };

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