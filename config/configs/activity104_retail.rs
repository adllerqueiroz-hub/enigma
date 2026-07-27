// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity104Retail {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "enemyTag")]
    pub enemy_tag: String,
    #[serde(rename = "equipRareWeight")]
    pub equip_rare_weight: String,
    pub level: i32,
    #[serde(rename = "retailEpisodeIdPool")]
    pub retail_episode_id_pool: String,
    pub stage: i32,
}
pub struct Activity104RetailTable {
    records: Vec<Activity104Retail>,
}

impl Activity104RetailTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity104Retail> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity104Retail] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity104Retail> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}