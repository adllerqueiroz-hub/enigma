// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightTask {
    pub condition: String,
    pub id: i32,
    #[serde(rename = "sysParam1")]
    pub sys_param1: String,
    #[serde(rename = "sysParam2")]
    pub sys_param2: String,
    #[serde(rename = "sysParam3")]
    pub sys_param3: String,
    #[serde(rename = "sysParam4")]
    pub sys_param4: String,
    #[serde(rename = "taskParam1")]
    pub task_param1: String,
    #[serde(rename = "taskParam2")]
    pub task_param2: String,
    #[serde(rename = "taskParam3")]
    pub task_param3: String,
    #[serde(rename = "taskParam4")]
    pub task_param4: String,
}
use std::collections::HashMap;

pub struct FightTaskTable {
    records: Vec<FightTask>,
    by_id: HashMap<i32, usize>,
}

impl FightTaskTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<FightTask> = if let Some(array) = value.as_array() {
            if array.len() >= 2 && array[1].is_array() {
                serde_json::from_value(array[1].clone())?
            } else {
                serde_json::from_value(value)?
            }
        } else {
            serde_json::from_value(value)?
        };

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
    pub fn get(&self, id: i32) -> Option<&FightTask> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[FightTask] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightTask> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}