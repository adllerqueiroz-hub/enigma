// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity216OnceBonus {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    #[serde(rename = "needFinishTaskNum")]
    pub need_finish_task_num: i32,
}
pub struct Activity216OnceBonusTable {
    records: Vec<Activity216OnceBonus>,
}

impl Activity216OnceBonusTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity216OnceBonus> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity216OnceBonus] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity216OnceBonus> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}