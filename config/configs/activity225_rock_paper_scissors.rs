// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity225RockPaperScissors {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "drawBonus")]
    pub draw_bonus: String,
    #[serde(rename = "loseBonus")]
    pub lose_bonus: String,
    pub times: i32,
    #[serde(rename = "winBonus")]
    pub win_bonus: String,
}
pub struct Activity225RockPaperScissorsTable {
    records: Vec<Activity225RockPaperScissors>,
}

impl Activity225RockPaperScissorsTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity225RockPaperScissors> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity225RockPaperScissors] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity225RockPaperScissors> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}