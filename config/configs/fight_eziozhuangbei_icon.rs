// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightEziozhuangbeiIcon {
    pub firsticon: String,
    pub name: String,
    pub secondicon: String,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "weaponId")]
    pub weapon_id: i32,
}
pub struct FightEziozhuangbeiIconTable {
    records: Vec<FightEziozhuangbeiIcon>,
}

impl FightEziozhuangbeiIconTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<FightEziozhuangbeiIcon> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[FightEziozhuangbeiIcon] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightEziozhuangbeiIcon> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}