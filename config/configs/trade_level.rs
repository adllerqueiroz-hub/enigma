// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeLevel {
    #[serde(rename = "addBlockMax")]
    pub add_block_max: i32,
    pub bonus: String,
    pub dimension: String,
    pub job: String,
    #[serde(rename = "jobCard")]
    pub job_card: String,
    pub level: i32,
    #[serde(rename = "levelUpNeedTask")]
    pub level_up_need_task: i32,
    #[serde(rename = "maxRestBuildingNum")]
    pub max_rest_building_num: i32,
    #[serde(rename = "maxTrainSlotCount")]
    pub max_train_slot_count: i32,
    #[serde(rename = "silenceBonus")]
    pub silence_bonus: String,
    #[serde(rename = "taskName")]
    pub task_name: String,
    #[serde(rename = "trainsRoundCount")]
    pub trains_round_count: i32,
    #[serde(rename = "unlockId")]
    pub unlock_id: String,
}
pub struct TradeLevelTable {
    records: Vec<TradeLevel>,
}

impl TradeLevelTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TradeLevel> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TradeLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TradeLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}