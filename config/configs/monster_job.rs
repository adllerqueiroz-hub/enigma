// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterJob {
    #[serde(rename = "addDmg_equip_super")]
    pub add_dmg_equip_super: i32,
    #[serde(rename = "addDmg_init_super")]
    pub add_dmg_init_super: i32,
    #[serde(rename = "addDmg_reson_super")]
    pub add_dmg_reson_super: i32,
    pub attack_base: i32,
    pub attack_base_coef: i32,
    pub attack_equip_base: i32,
    #[serde(rename = "criDef_equip_super")]
    pub cri_def_equip_super: i32,
    #[serde(rename = "criDef_init_super")]
    pub cri_def_init_super: i32,
    #[serde(rename = "criDef_reson_super")]
    pub cri_def_reson_super: i32,
    #[serde(rename = "criDmg_equip_super")]
    pub cri_dmg_equip_super: i32,
    #[serde(rename = "criDmg_init_super")]
    pub cri_dmg_init_super: i32,
    #[serde(rename = "criDmg_reson_super")]
    pub cri_dmg_reson_super: i32,
    pub cri_equip_super: i32,
    pub cri_init_super: i32,
    pub cri_reson_super: i32,
    pub defense_base: i32,
    pub defense_base_coef: i32,
    pub defense_equip_base: i32,
    #[serde(rename = "dropDmg_equip_super")]
    pub drop_dmg_equip_super: i32,
    #[serde(rename = "dropDmg_init_super")]
    pub drop_dmg_init_super: i32,
    #[serde(rename = "dropDmg_reson_super")]
    pub drop_dmg_reson_super: i32,
    pub id: i32,
    pub life_base: i32,
    pub life_base_coef: i32,
    pub life_equip_base: i32,
    pub mdefense_base: i32,
    pub mdefense_base_coef: i32,
    pub mdefense_equip_base: i32,
    pub recri_equip_super: i32,
    pub recri_init_super: i32,
    pub recri_reson_super: i32,
    pub technic_base: i32,
    pub technic_base_coef: i32,
    pub technic_equip_base: i32,
}
use std::collections::HashMap;

pub struct MonsterJobTable {
    records: Vec<MonsterJob>,
    by_id: HashMap<i32, usize>,
}

impl MonsterJobTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<MonsterJob> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&MonsterJob> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[MonsterJob] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, MonsterJob> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}