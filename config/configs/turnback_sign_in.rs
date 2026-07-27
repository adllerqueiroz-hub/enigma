// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnbackSignIn {
    pub bonus: String,
    #[serde(rename = "characterId")]
    pub character_id: i32,
    pub content: String,
    pub day: i32,
    pub name: String,
    #[serde(rename = "turnbackId")]
    pub turnback_id: i32,
}
pub struct TurnbackSignInTable {
    records: Vec<TurnbackSignIn>,
}

impl TurnbackSignInTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TurnbackSignIn> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TurnbackSignIn] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TurnbackSignIn> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}