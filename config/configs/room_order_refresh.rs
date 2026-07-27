// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomOrderRefresh {
    #[serde(rename = "finishLimitDaily")]
    pub finish_limit_daily: i32,
    pub level: i32,
    #[serde(rename = "meanwhileWholesaleNum")]
    pub meanwhile_wholesale_num: i32,
    #[serde(rename = "qualityWeight")]
    pub quality_weight: String,
    #[serde(rename = "wholesaleGoodsWeight")]
    pub wholesale_goods_weight: String,
    #[serde(rename = "wholesaleRevenueWeeklyLimit")]
    pub wholesale_revenue_weekly_limit: i32,
}
pub struct RoomOrderRefreshTable {
    records: Vec<RoomOrderRefresh>,
}

impl RoomOrderRefreshTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<RoomOrderRefresh> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[RoomOrderRefresh] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomOrderRefresh> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}