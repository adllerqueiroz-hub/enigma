// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actvity205Stage {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub desc: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    pub icon: String,
    pub name: String,
    #[serde(rename = "ruleDesc")]
    pub rule_desc: String,
    #[serde(rename = "ruleTitle")]
    pub rule_title: String,
    #[serde(rename = "stageId")]
    pub stage_id: i32,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "targetDesc")]
    pub target_desc: String,
    pub times: i32,
}
pub struct Actvity205StageTable {
    records: Vec<Actvity205Stage>,
}

impl Actvity205StageTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Actvity205Stage> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Actvity205Stage] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actvity205Stage> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}