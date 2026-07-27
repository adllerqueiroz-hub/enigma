// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity166Info {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "infoId")]
    pub info_id: i32,
    #[serde(rename = "initContent")]
    pub init_content: String,
    pub name: String,
    #[serde(rename = "nameEn")]
    pub name_en: String,
    #[serde(rename = "reportPic")]
    pub report_pic: String,
    #[serde(rename = "reportRes")]
    pub report_res: String,
    #[serde(rename = "unlockDes")]
    pub unlock_des: String,
    #[serde(rename = "unlockParam")]
    pub unlock_param: String,
    #[serde(rename = "unlockType")]
    pub unlock_type: i32,
}
pub struct Activity166InfoTable {
    records: Vec<Activity166Info>,
}

impl Activity166InfoTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity166Info> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity166Info] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity166Info> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}