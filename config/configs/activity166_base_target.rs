// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166BaseTarget {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "baseId")]
    pub base_id: i32,
    pub score: String,
    #[serde(rename = "targetDesc")]
    pub target_desc: String,
    #[serde(rename = "targetId")]
    pub target_id: i32,
    #[serde(rename = "targetParam")]
    pub target_param: String,
    #[serde(rename = "targetType")]
    pub target_type: i32,
}
pub struct Activity166BaseTargetTable {
    records: Vec<Activity166BaseTarget>,
}

impl Activity166BaseTargetTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity166BaseTarget> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity166BaseTarget] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166BaseTarget> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}