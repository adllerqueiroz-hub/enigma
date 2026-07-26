// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity125 {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    pub clientbonus: String,
    pub frequency: String,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    pub id: i32,
    #[serde(rename = "initFrequency")]
    pub init_frequency: i32,
    pub key: String,
    pub musictime: i32,
    pub name: String,
    #[serde(rename = "openDay")]
    pub open_day: i32,
    #[serde(rename = "preId")]
    pub pre_id: i32,
    pub sourceid: i32,
    #[serde(rename = "targetFrequency")]
    pub target_frequency: i32,
    #[serde(rename = "taskId")]
    pub task_id: i32,
    pub text: String,
    pub time: i32,
}
use std::collections::HashMap;

pub struct Activity125Table {
    records: Vec<Activity125>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl Activity125Table {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let value: serde_json::Value = serde_json::from_str(&json)?;

        let records: Vec<Activity125> = if let Some(array) = value.as_array() {
            if array.len() >= 2 && array[1].is_array() {
                serde_json::from_value(array[1].clone())?
            } else {
                serde_json::from_value(value)?
            }
        } else {
            serde_json::from_value(value)?
        };

        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();

        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.group_id).or_default().push(idx);
        }

        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Activity125> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &'_ Activity125> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|idxs| idxs.iter())
            .map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Activity125] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity125> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}