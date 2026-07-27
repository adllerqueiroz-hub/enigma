// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity206Desc {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub desc: String,
    pub icon: String,
    pub name: String,
    #[serde(rename = "ruleDesc")]
    pub rule_desc: String,
    #[serde(rename = "ruleTitle")]
    pub rule_title: String,
    #[serde(rename = "stageId")]
    pub stage_id: i32,
    #[serde(rename = "targetDesc")]
    pub target_desc: String,
}
pub struct Activity206DescTable {
    records: Vec<Activity206Desc>,
}

impl Activity206DescTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity206Desc> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity206Desc] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity206Desc> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}