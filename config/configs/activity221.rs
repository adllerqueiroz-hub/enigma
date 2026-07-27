// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity221 {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "constId")]
    pub const_id: i32,
    #[serde(rename = "doubleSixRate")]
    pub double_six_rate: i32,
    #[serde(rename = "heroExtraDesc")]
    pub hero_extra_desc: String,
    #[serde(rename = "initWeight")]
    pub init_weight: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "poolId")]
    pub pool_id: i32,
    #[serde(rename = "summonTimes")]
    pub summon_times: i32,
}
pub struct Activity221Table {
    records: Vec<Activity221>,
}

impl Activity221Table {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity221> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity221] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity221> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}