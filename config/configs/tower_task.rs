// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerTask {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    #[serde(rename = "bonusMail")]
    pub bonus_mail: i32,
    pub desc: String,
    pub id: i32,
    #[serde(rename = "isKeyReward")]
    pub is_key_reward: i32,
    #[serde(rename = "isOnline")]
    pub is_online: i32,
    #[serde(rename = "jumpId")]
    pub jump_id: i32,
    #[serde(rename = "listenerParam")]
    pub listener_param: String,
    #[serde(rename = "listenerType")]
    pub listener_type: String,
    #[serde(rename = "maxProgress")]
    pub max_progress: i32,
    #[serde(rename = "minType")]
    pub min_type: String,
    pub name: String,
    #[serde(rename = "openLimit")]
    pub open_limit: String,
    #[serde(rename = "taskGroupId")]
    pub task_group_id: i32,
}
use std::collections::HashMap;

pub struct TowerTaskTable {
    records: Vec<TowerTask>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl TowerTaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerTask> = crate::load_rows(path)?;

        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();

        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.task_group_id).or_default().push(idx);
        }

        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&TowerTask> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &'_ TowerTask> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|idxs| idxs.iter())
            .map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[TowerTask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerTask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}