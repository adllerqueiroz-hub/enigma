// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity225Npc {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "eggTxt")]
    pub egg_txt: String,
    #[serde(rename = "interactiveId")]
    pub interactive_id: i32,
    #[serde(rename = "keepTime")]
    pub keep_time: i32,
    #[serde(rename = "npcId")]
    pub npc_id: i32,
    #[serde(rename = "npcName")]
    pub npc_name: String,
    #[serde(rename = "npcRace")]
    pub npc_race: i32,
    #[serde(rename = "npcType")]
    pub npc_type: i32,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
    #[serde(rename = "titleId")]
    pub title_id: i32,
    pub weight: i32,
}
pub struct Activity225NpcTable {
    records: Vec<Activity225Npc>,
}

impl Activity225NpcTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity225Npc> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity225Npc] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity225Npc> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}