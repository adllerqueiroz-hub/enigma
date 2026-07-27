// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufactureBuildingTradeLevel {
    #[serde(rename = "maxCritterCount")]
    pub max_critter_count: i32,
    #[serde(rename = "tradeGroupId")]
    pub trade_group_id: i32,
    #[serde(rename = "tradeLevel")]
    pub trade_level: i32,
}
use std::collections::HashMap;

pub struct ManufactureBuildingTradeLevelTable {
    records: Vec<ManufactureBuildingTradeLevel>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl ManufactureBuildingTradeLevelTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<ManufactureBuildingTradeLevel> = crate::load_rows(path)?;

        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();

        for (idx, record) in records.iter().enumerate() {
            by_group.entry(record.trade_group_id).or_default().push(idx);
        }

        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &'_ ManufactureBuildingTradeLevel> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|idxs| idxs.iter())
            .map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[ManufactureBuildingTradeLevel] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, ManufactureBuildingTradeLevel> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}