// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FightDevice {
    pub id: i32,
    #[serde(rename = "powerSkill")]
    pub power_skill: String,
    pub skill1: String,
    pub skill2: String,
    #[serde(rename = "specialPowerSkill")]
    pub special_power_skill: String,
    #[serde(rename = "uniqueSkill")]
    pub unique_skill: String,
}
use std::collections::HashMap;

pub struct FightDeviceTable {
    records: Vec<FightDevice>,
    by_id: HashMap<i32, usize>,
}

impl FightDeviceTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<FightDevice> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&FightDevice> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[FightDevice] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, FightDevice> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}