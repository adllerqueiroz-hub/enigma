// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actvity204Task {
    #[serde(rename = "acceptStage")]
    pub accept_stage: String,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    #[serde(rename = "bonusMail")]
    pub bonus_mail: i32,
    pub desc: String,
    #[serde(rename = "durationHour")]
    pub duration_hour: i32,
    #[serde(rename = "durationLimitActivityId")]
    pub duration_limit_activity_id: i32,
    pub id: i32,
    #[serde(rename = "isOnline")]
    pub is_online: i32,
    #[serde(rename = "jumpId")]
    pub jump_id: i32,
    #[serde(rename = "listenerParam")]
    pub listener_param: String,
    #[serde(rename = "listenerType")]
    pub listener_type: String,
    #[serde(rename = "loopType")]
    pub loop_type: i32,
    #[serde(rename = "maxProgress")]
    pub max_progress: i32,
    #[serde(rename = "minType")]
    pub min_type: String,
    pub missionorder: i32,
    pub name: String,
    #[serde(rename = "openLimit")]
    pub open_limit: String,
    pub prepose: String,
    #[serde(rename = "realPrepose")]
    pub real_prepose: String,
    pub secretornot: i32,
}
use std::collections::HashMap;

pub struct Actvity204TaskTable {
    records: Vec<Actvity204Task>,
    by_id: HashMap<i32, usize>,
}

impl Actvity204TaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Actvity204Task> = crate::load_rows(path)?;

        let mut by_id = HashMap::with_capacity(records.len());

        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }

        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Actvity204Task> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Actvity204Task] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actvity204Task> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}