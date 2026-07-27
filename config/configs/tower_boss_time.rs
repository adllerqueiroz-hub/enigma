// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerBossTime {
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "isOnline")]
    pub is_online: i32,
    #[serde(rename = "isPermanent")]
    pub is_permanent: i32,
    pub round: i32,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "taskEndTime")]
    pub task_end_time: String,
    #[serde(rename = "taskGroupId")]
    pub task_group_id: i32,
    #[serde(rename = "towerId")]
    pub tower_id: i32,
}
use std::collections::HashMap;

pub struct TowerBossTimeTable {
    records: Vec<TowerBossTime>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl TowerBossTimeTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerBossTime> = crate::load_rows(path)?;

        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();

        for (idx, record) in records.iter().enumerate() {
            by_group.entry(record.task_group_id).or_default().push(idx);
        }

        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &'_ TowerBossTime> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|idxs| idxs.iter())
            .map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[TowerBossTime] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerBossTime> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}