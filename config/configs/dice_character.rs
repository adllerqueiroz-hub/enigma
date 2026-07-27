// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiceCharacter {
    pub desc: String,
    pub dicelist: String,
    pub hp: i32,
    pub icon: i32,
    pub id: i32,
    pub name: String,
    pub power: i32,
    #[serde(rename = "powerSkill")]
    pub power_skill: i32,
    #[serde(rename = "relicIds")]
    pub relic_ids: String,
    #[serde(rename = "resetTimes")]
    pub reset_times: i32,
    pub skilllist: String,
}
use std::collections::HashMap;

pub struct DiceCharacterTable {
    records: Vec<DiceCharacter>,
    by_id: HashMap<i32, usize>,
}

impl DiceCharacterTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<DiceCharacter> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&DiceCharacter> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[DiceCharacter] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, DiceCharacter> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}