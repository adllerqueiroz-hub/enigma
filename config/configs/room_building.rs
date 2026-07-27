// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomBuilding {
    #[serde(rename = "alphaThreshold")]
    pub alpha_threshold: i32,
    #[serde(rename = "areaId")]
    pub area_id: i32,
    #[serde(rename = "audioExtendIds")]
    pub audio_extend_ids: String,
    #[serde(rename = "audioExtendType")]
    pub audio_extend_type: i32,
    #[serde(rename = "buildDegree")]
    pub build_degree: i32,
    #[serde(rename = "buildingShowType")]
    pub building_show_type: i32,
    #[serde(rename = "buildingType")]
    pub building_type: i32,
    #[serde(rename = "canExchange")]
    pub can_exchange: bool,
    #[serde(rename = "canLevelUp")]
    pub can_level_up: bool,
    #[serde(rename = "canPlaceBlock")]
    pub can_place_block: String,
    pub center: String,
    #[serde(rename = "costResource")]
    pub cost_resource: String,
    pub crossload: i32,
    pub desc: String,
    #[serde(rename = "dragUpHeight")]
    pub drag_up_height: i32,
    #[serde(rename = "gatherDesc")]
    pub gather_desc: String,
    pub icon: String,
    pub id: i32,
    #[serde(rename = "isAreaMainBuilding")]
    pub is_area_main_building: bool,
    #[serde(rename = "linkBlock")]
    pub link_block: String,
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "numLimit")]
    pub num_limit: i32,
    pub offset: String,
    pub path: String,
    #[serde(rename = "placeAudio")]
    pub place_audio: i32,
    #[serde(rename = "produceDesc")]
    pub produce_desc: String,
    pub rare: i32,
    pub reflerction: i32,
    #[serde(rename = "replaceBlock")]
    pub replace_block: String,
    #[serde(rename = "rewardIcon")]
    pub reward_icon: String,
    pub rotate: i32,
    pub sound: i32,
    pub sources: String,
    #[serde(rename = "sourcesType")]
    pub sources_type: String,
    #[serde(rename = "uiScale")]
    pub ui_scale: i32,
    #[serde(rename = "useDesc")]
    pub use_desc: String,
    #[serde(rename = "vehicleId")]
    pub vehicle_id: i32,
    #[serde(rename = "vehicleType")]
    pub vehicle_type: i32,
}
use std::collections::HashMap;

pub struct RoomBuildingTable {
    records: Vec<RoomBuilding>,
    by_id: HashMap<i32, usize>,
}

impl RoomBuildingTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<RoomBuilding> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&RoomBuilding> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[RoomBuilding] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, RoomBuilding> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}