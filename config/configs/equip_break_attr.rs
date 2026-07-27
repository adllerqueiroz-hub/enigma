// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipBreakAttr {
    pub absorb: i32,
    #[serde(rename = "addDmg")]
    pub add_dmg: i32,
    pub attack: i32,
    #[serde(rename = "breakLevel")]
    pub break_level: i32,
    pub clutch: i32,
    pub cri: i32,
    #[serde(rename = "criDef")]
    pub cri_def: i32,
    #[serde(rename = "criDmg")]
    pub cri_dmg: i32,
    pub def: i32,
    #[serde(rename = "defenseIgnore")]
    pub defense_ignore: i32,
    #[serde(rename = "dropDmg")]
    pub drop_dmg: i32,
    pub heal: i32,
    pub hp: i32,
    pub id: i32,
    pub mdef: i32,
    #[serde(rename = "normalSkillRate")]
    pub normal_skill_rate: i32,
    pub recri: i32,
    pub revive: i32,
}
use std::collections::HashMap;

pub struct EquipBreakAttrTable {
    records: Vec<EquipBreakAttr>,
    by_id: HashMap<i32, usize>,
}

impl EquipBreakAttrTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<EquipBreakAttr> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&EquipBreakAttr> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[EquipBreakAttr] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, EquipBreakAttr> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}