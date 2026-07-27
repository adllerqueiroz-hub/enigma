// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hero3124SkillTalent {
    #[serde(rename = "additionalFieldDesc")]
    pub additional_field_desc: String,
    #[serde(rename = "additionalFieldDesc1")]
    pub additional_field_desc1: String,
    #[serde(rename = "additionalFieldDesc2")]
    pub additional_field_desc2: String,
    #[serde(rename = "additionalFieldDesc3")]
    pub additional_field_desc3: String,
    #[serde(rename = "additionalFieldDesc4")]
    pub additional_field_desc4: String,
    #[serde(rename = "additionalFieldDesc5")]
    pub additional_field_desc5: String,
    pub desc: String,
    pub desc1: String,
    pub desc2: String,
    pub desc3: String,
    pub desc4: String,
    pub desc5: String,
    #[serde(rename = "exchangeSkills0")]
    pub exchange_skills0: String,
    #[serde(rename = "exchangeSkills1")]
    pub exchange_skills1: String,
    #[serde(rename = "exchangeSkills2")]
    pub exchange_skills2: String,
    #[serde(rename = "exchangeSkills3")]
    pub exchange_skills3: String,
    #[serde(rename = "exchangeSkills4")]
    pub exchange_skills4: String,
    #[serde(rename = "exchangeSkills5")]
    pub exchange_skills5: String,
    #[serde(rename = "fieldActivateDesc")]
    pub field_activate_desc: String,
    #[serde(rename = "fieldActivateDesc1")]
    pub field_activate_desc1: String,
    #[serde(rename = "fieldActivateDesc2")]
    pub field_activate_desc2: String,
    #[serde(rename = "fieldActivateDesc3")]
    pub field_activate_desc3: String,
    #[serde(rename = "fieldActivateDesc4")]
    pub field_activate_desc4: String,
    #[serde(rename = "fieldActivateDesc5")]
    pub field_activate_desc5: String,
    #[serde(rename = "fieldDesc")]
    pub field_desc: String,
    #[serde(rename = "fieldDesc1")]
    pub field_desc1: String,
    #[serde(rename = "fieldDesc2")]
    pub field_desc2: String,
    #[serde(rename = "fieldDesc3")]
    pub field_desc3: String,
    #[serde(rename = "fieldDesc4")]
    pub field_desc4: String,
    #[serde(rename = "fieldDesc5")]
    pub field_desc5: String,
    #[serde(rename = "fieldName")]
    pub field_name: String,
    pub icon: String,
    pub level: i32,
    pub name: String,
    #[serde(rename = "newSkills0")]
    pub new_skills0: String,
    #[serde(rename = "newSkills1")]
    pub new_skills1: String,
    #[serde(rename = "newSkills2")]
    pub new_skills2: String,
    #[serde(rename = "newSkills3")]
    pub new_skills3: String,
    #[serde(rename = "newSkills4")]
    pub new_skills4: String,
    #[serde(rename = "newSkills5")]
    pub new_skills5: String,
    pub sub: i32,
    #[serde(rename = "talentId")]
    pub talent_id: i32,
}
pub struct Hero3124SkillTalentTable {
    records: Vec<Hero3124SkillTalent>,
}

impl Hero3124SkillTalentTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<Hero3124SkillTalent> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Hero3124SkillTalent] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Hero3124SkillTalent> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}