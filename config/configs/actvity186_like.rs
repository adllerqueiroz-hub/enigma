// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actvity186Like {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub basevalueornot: i32,
    pub icon: String,
    pub name: String,
    pub nameen: String,
    #[serde(rename = "type")]
    pub r#type: i32,
}
pub struct Actvity186LikeTable {
    records: Vec<Actvity186Like>,
}

impl Actvity186LikeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Actvity186Like> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Actvity186Like] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actvity186Like> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}