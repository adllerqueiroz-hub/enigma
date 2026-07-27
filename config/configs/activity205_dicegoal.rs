// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity205Dicegoal {
    #[serde(rename = "bindingDice")]
    pub binding_dice: String,
    #[serde(rename = "failRewardId")]
    pub fail_reward_id: i32,
    #[serde(rename = "goalRange")]
    pub goal_range: String,
    pub goaldesc: String,
    pub goalname: String,
    #[serde(rename = "hardType")]
    pub hard_type: i32,
    #[serde(rename = "iconRes")]
    pub icon_res: String,
    pub id: i32,
    pub weight: i32,
    #[serde(rename = "winRewardId")]
    pub win_reward_id: i32,
}
use std::collections::HashMap;

pub struct Activity205DicegoalTable {
    records: Vec<Activity205Dicegoal>,
    by_id: HashMap<i32, usize>,
}

impl Activity205DicegoalTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity205Dicegoal> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Activity205Dicegoal> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Activity205Dicegoal] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity205Dicegoal> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}