// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreGoods {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bigImg")]
    pub big_img: String,
    pub bindgoodid: i32,
    #[serde(rename = "buyLevel")]
    pub buy_level: i32,
    pub cost: String,
    pub cost2: String,
    #[serde(rename = "deductionItem")]
    pub deduction_item: String,
    #[serde(rename = "discountItem")]
    pub discount_item: String,
    pub duration: i32,
    pub id: i32,
    #[serde(rename = "isAdvancedSkin")]
    pub is_advanced_skin: bool,
    #[serde(rename = "isOnline")]
    pub is_online: bool,
    #[serde(rename = "islinkageSkin")]
    pub islinkage_skin: bool,
    #[serde(rename = "jumpId")]
    pub jump_id: i32,
    #[serde(rename = "logoRoots")]
    pub logo_roots: String,
    #[serde(rename = "maxBuyCount")]
    pub max_buy_count: i32,
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "needEpisodeId")]
    pub need_episode_id: i32,
    #[serde(rename = "needTopup")]
    pub need_topup: i32,
    #[serde(rename = "needWeekwalkLayer")]
    pub need_weekwalk_layer: i32,
    #[serde(rename = "newEndTime")]
    pub new_end_time: String,
    #[serde(rename = "newStartTime")]
    pub new_start_time: String,
    #[serde(rename = "notShowInRecommend")]
    pub not_show_in_recommend: bool,
    #[serde(rename = "offTag")]
    pub off_tag: String,
    #[serde(rename = "offlineTime")]
    pub offline_time: String,
    #[serde(rename = "onlineTime")]
    pub online_time: String,
    #[serde(rename = "openLevel")]
    pub open_level: i32,
    pub order: i32,
    #[serde(rename = "originalCost")]
    pub original_cost: i32,
    #[serde(rename = "preGoodsId")]
    pub pre_goods_id: i32,
    pub product: String,
    pub reduction: String,
    #[serde(rename = "refreshTime")]
    pub refresh_time: i32,
    #[serde(rename = "showBg")]
    pub show_bg: i32,
    #[serde(rename = "showLinkageLogo")]
    pub show_linkage_logo: bool,
    #[serde(rename = "showRefreshTime")]
    pub show_refresh_time: i32,
    #[serde(rename = "skinLevel")]
    pub skin_level: i32,
    pub slogan: String,
    #[serde(rename = "specialofferItem")]
    pub specialoffer_item: String,
    #[serde(rename = "spineParams")]
    pub spine_params: String,
    #[serde(rename = "storeId")]
    pub store_id: String,
    pub underlay: String,
}
use std::collections::HashMap;

pub struct StoreGoodsTable {
    records: Vec<StoreGoods>,
    by_id: HashMap<i32, usize>,
}

impl StoreGoodsTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<StoreGoods> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&StoreGoods> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[StoreGoods] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, StoreGoods> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}