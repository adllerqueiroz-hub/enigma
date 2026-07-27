// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Critter {
    #[serde(rename = "attributeIncrRate")]
    pub attribute_incr_rate: String,
    #[serde(rename = "banishBonus")]
    pub banish_bonus: i32,
    #[serde(rename = "baseAttribute")]
    pub base_attribute: String,
    pub catalogue: i32,
    pub desc: String,
    #[serde(rename = "eventTimes")]
    pub event_times: String,
    #[serde(rename = "foodLike")]
    pub food_like: String,
    pub icon: i32,
    pub id: i32,
    pub line: String,
    #[serde(rename = "mutateSkin")]
    pub mutate_skin: i32,
    pub name: String,
    #[serde(rename = "normalSkin")]
    pub normal_skin: i32,
    #[serde(rename = "raceTag")]
    pub race_tag: String,
    pub rare: i32,
    pub relation: String,
    #[serde(rename = "specialRate")]
    pub special_rate: i32,
    pub story: String,
    #[serde(rename = "trainTime")]
    pub train_time: i32,
}
use std::collections::HashMap;

pub struct CritterTable {
    records: Vec<Critter>,
    by_id: HashMap<i32, usize>,
}

impl CritterTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Critter> = crate::load_rows(path)?;

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
    pub fn get(&self, id: i32) -> Option<&Critter> {
        self.by_id.get(&id).map(|&i| &self.records[i])
    }

    #[inline]
    pub fn all(&self) -> &[Critter] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Critter> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}