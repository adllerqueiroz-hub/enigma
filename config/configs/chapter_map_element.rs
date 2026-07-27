// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterMapElement {
    #[serde(rename = "acceptText")]
    pub accept_text: String,
    pub condition: String,
    pub desc: String,
    #[serde(rename = "dispatchingText")]
    pub dispatching_text: String,
    pub effect: String,
    #[serde(rename = "finishText")]
    pub finish_text: String,
    #[serde(rename = "flagText")]
    pub flag_text: String,
    pub fragment: i32,
    #[serde(rename = "holeSize")]
    pub hole_size: String,
    pub id: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "offsetPos")]
    pub offset_pos: String,
    pub param: String,
    #[serde(rename = "paramCn")]
    pub param_cn: String,
    #[serde(rename = "paramLang")]
    pub param_lang: String,
    #[serde(rename = "permanentReward")]
    pub permanent_reward: String,
    pub pos: String,
    pub res: String,
    #[serde(rename = "resScale")]
    pub res_scale: f32,
    #[serde(rename = "retroReward")]
    pub retro_reward: String,
    pub reward: String,
    #[serde(rename = "rewardPoint")]
    pub reward_point: i32,
    #[serde(rename = "showArrow")]
    pub show_arrow: i32,
    #[serde(rename = "showCamera")]
    pub show_camera: i32,
    #[serde(rename = "tipOffsetPos")]
    pub tip_offset_pos: String,
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: i32,
}
use std::collections::HashMap;

pub struct ChapterMapElementTable {
    records: Vec<ChapterMapElement>,
    by_id: HashMap<i32, usize>,
}

impl ChapterMapElementTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<ChapterMapElement> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&ChapterMapElement> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[ChapterMapElement] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, ChapterMapElement> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}