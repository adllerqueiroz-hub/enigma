// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity152 {
    #[serde(rename = "acceptDate")]
    pub accept_date: String,
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    pub bonus: String,
    pub dialog: String,
    #[serde(rename = "presentIcon")]
    pub present_icon: String,
    #[serde(rename = "presentId")]
    pub present_id: i32,
    #[serde(rename = "presentName")]
    pub present_name: String,
    #[serde(rename = "presentSign")]
    pub present_sign: String,
    #[serde(rename = "roleIcon")]
    pub role_icon: String,
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "roleNameEn")]
    pub role_name_en: String,
}
pub struct Activity152Table {
    records: Vec<Activity152>,
}

impl Activity152Table {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity152> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity152] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity152> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}