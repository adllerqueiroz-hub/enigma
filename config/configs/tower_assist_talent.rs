// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TowerAssistTalent {
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    #[serde(rename = "bossPassiveSkills")]
    pub boss_passive_skills: String,
    pub consume: i32,
    #[serde(rename = "extraRule")]
    pub extra_rule: String,
    #[serde(rename = "heroPassiveSkills")]
    pub hero_passive_skills: String,
    #[serde(rename = "isBigNode")]
    pub is_big_node: i32,
    #[serde(rename = "nodeDesc")]
    pub node_desc: String,
    #[serde(rename = "nodeGroup")]
    pub node_group: i32,
    #[serde(rename = "nodeId")]
    pub node_id: i32,
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(rename = "nodeType")]
    pub node_type: i32,
    pub position: String,
    #[serde(rename = "preNodeIds")]
    pub pre_node_ids: String,
    #[serde(rename = "startNode")]
    pub start_node: i32,
}
pub struct TowerAssistTalentTable {
    records: Vec<TowerAssistTalent>,
}

impl TowerAssistTalentTable {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let records: Vec<TowerAssistTalent> = crate::load_rows(path)?;

        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[TowerAssistTalent] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, TowerAssistTalent> {
        self.records.iter()
    }

    pub fn len(&self) -> usize { self.records.len() }
    pub fn is_empty(&self) -> bool { self.records.is_empty() }
}