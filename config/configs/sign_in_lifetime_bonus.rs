// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignInLifetimeBonus {
    pub bonus: String,
    pub logindaysid: i32,
    pub stageid: i32,
    pub stagetitle: String,
}
pub struct SignInLifetimeBonusTable {
    records: Vec<SignInLifetimeBonus>,
}

impl SignInLifetimeBonusTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<SignInLifetimeBonus> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[SignInLifetimeBonus] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SignInLifetimeBonus> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}