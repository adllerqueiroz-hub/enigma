// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bp {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "bpId")]
    pub bp_id: i32,
    #[serde(rename = "bpSkinDesc")]
    pub bp_skin_desc: String,
    #[serde(rename = "bpSkinEnNametxt")]
    pub bp_skin_en_nametxt: String,
    #[serde(rename = "bpSkinNametxt")]
    pub bp_skin_nametxt: String,
    pub bpviewicon: String,
    pub bpviewpos: String,
    #[serde(rename = "chargeId1")]
    pub charge_id1: i32,
    #[serde(rename = "chargeId1to2")]
    pub charge_id1to2: i32,
    #[serde(rename = "chargeId2")]
    pub charge_id2: i32,
    #[serde(rename = "expLevelUp")]
    pub exp_level_up: i32,
    #[serde(rename = "expUpShow")]
    pub exp_up_show: bool,
    #[serde(rename = "isSp")]
    pub is_sp: bool,
    pub name: String,
    #[serde(rename = "payStatus1Bonus")]
    pub pay_status1_bonus: String,
    #[serde(rename = "payStatus2AddLevel")]
    pub pay_status2_add_level: i32,
    #[serde(rename = "payStatus2Bonus")]
    pub pay_status2_bonus: String,
    #[serde(rename = "promptDays")]
    pub prompt_days: i32,
    #[serde(rename = "showBonus")]
    pub show_bonus: String,
    #[serde(rename = "showBonusDate")]
    pub show_bonus_date: String,
    #[serde(rename = "specialBonus")]
    pub special_bonus: i32,
    #[serde(rename = "specialPropDesc")]
    pub special_prop_desc: String,
    #[serde(rename = "weekLimitTimes")]
    pub week_limit_times: i32,
}
pub struct BpTable {
    records: Vec<Bp>,
}

impl BpTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Bp> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Bp] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Bp> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}