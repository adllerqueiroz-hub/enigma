// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equip {
    pub amplify: i32,
    #[serde(rename = "canShowHandbook")]
    pub can_show_handbook: String,
    #[serde(rename = "characterId")]
    pub character_id: String,
    pub desc: String,
    pub icon: String,
    pub id: i32,
    #[serde(rename = "isExpEquip")]
    pub is_exp_equip: i32,
    #[serde(rename = "isSpRefine")]
    pub is_sp_refine: i32,
    pub name: String,
    pub name_en: String,
    pub rare: i32,
    #[serde(rename = "skillName")]
    pub skill_name: String,
    #[serde(rename = "skillType")]
    pub skill_type: i32,
    pub sources: String,
    #[serde(rename = "strengthType")]
    pub strength_type: i32,
    pub tag: String,
    #[serde(rename = "upperLimit")]
    pub upper_limit: i32,
    #[serde(rename = "useDesc")]
    pub use_desc: String,
    #[serde(rename = "useSpRefine")]
    pub use_sp_refine: String,
}
use std::collections::HashMap;

pub struct EquipTable {
    records: Vec<Equip>,
    by_id: HashMap<i32, usize>,
}

impl EquipTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Equip> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Equip> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Equip] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Equip> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}