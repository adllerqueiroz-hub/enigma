// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friendless {
    pub friendliness: i32,
    pub percentage: i32,
}
pub struct FriendlessTable {
    records: Vec<Friendless>,
}

impl FriendlessTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Friendless> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Friendless] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Friendless> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}