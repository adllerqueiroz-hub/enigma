use crate::{
    GameDB, tower_assist_develop::TowerAssistDevelop, tower_assist_talent::TowerAssistTalent,
    tower_talent_plan::TowerTalentPlan,
};

impl GameDB {
    pub fn tower_assist_talent(&self, boss_id: i32, node_id: i32) -> Option<&TowerAssistTalent> {
        self.tower_assist_talents(boss_id)
            .find(|row| row.node_id == node_id)
    }

    pub fn tower_assist_talents(&self, boss_id: i32) -> impl Iterator<Item = &TowerAssistTalent> {
        self.tower_assist_talent
            .iter()
            .filter(move |row| row.boss_id == boss_id)
    }

    pub fn tower_assist_development(
        &self,
        boss_id: i32,
    ) -> impl Iterator<Item = &TowerAssistDevelop> {
        self.tower_assist_develop
            .iter()
            .filter(move |row| row.boss_id == boss_id)
    }

    pub fn tower_talent_plan(&self, boss_id: i32, plan_id: i32) -> Option<&TowerTalentPlan> {
        self.tower_talent_plan
            .iter()
            .find(|row| row.boss_id == boss_id && row.plan_id == plan_id)
    }

    pub fn tower_talent_plans(&self, boss_id: i32) -> impl Iterator<Item = &TowerTalentPlan> {
        self.tower_talent_plan
            .iter()
            .filter(move |row| row.boss_id == boss_id)
    }
}
