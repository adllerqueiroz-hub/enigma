// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterGroup {
    #[serde(rename = "aiId")]
    pub ai_id: i32,
    #[serde(rename = "appearCameraPos")]
    pub appear_camera_pos: String,
    #[serde(rename = "appearMonsterId")]
    pub appear_monster_id: i32,
    #[serde(rename = "appearTimeline")]
    pub appear_timeline: String,
    pub bgm: i32,
    #[serde(rename = "bossId")]
    pub boss_id: String,
    pub id: i32,
    pub monster: String,
    #[serde(rename = "sp2Monster")]
    pub sp2_monster: String,
    #[serde(rename = "sp2Supporter")]
    pub sp2_supporter: String,
    #[serde(rename = "spMonster")]
    pub sp_monster: String,
    #[serde(rename = "spSupporter")]
    pub sp_supporter: String,
    #[serde(rename = "stanceId")]
    pub stance_id: i32,
}
use std::collections::HashMap;

pub struct MonsterGroupTable {
    records: Vec<MonsterGroup>,
    by_id: HashMap<i32, usize>,
}

impl MonsterGroupTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<MonsterGroup> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&MonsterGroup> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[MonsterGroup] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, MonsterGroup> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}