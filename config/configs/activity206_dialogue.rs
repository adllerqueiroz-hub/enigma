// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity206Dialogue {
    #[serde(rename = "activityId")]
    pub activity_id: i32,
    #[serde(rename = "chaseId")]
    pub chase_id: i32,
    pub dialog: String,
    #[serde(rename = "roleIcon")]
    pub role_icon: String,
    #[serde(rename = "roleName")]
    pub role_name: String,
    #[serde(rename = "roleNameEn")]
    pub role_name_en: String,
}
pub struct Activity206DialogueTable {
    records: Vec<Activity206Dialogue>,
}

impl Activity206DialogueTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Activity206Dialogue> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Activity206Dialogue] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Activity206Dialogue> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}