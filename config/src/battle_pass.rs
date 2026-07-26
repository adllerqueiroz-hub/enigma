use crate::{GameDB, bp::Bp, bp_lv_bonus::BpLvBonus, bp_task::BpTask};

impl GameDB {
    pub fn battle_pass(&self, bp_id: i32) -> Option<&Bp> {
        self.bp.iter().find(|row| row.bp_id == bp_id)
    }

    pub fn battle_pass_bonuses(&self, bp_id: i32) -> impl Iterator<Item = &BpLvBonus> {
        self.bp_lv_bonus
            .iter()
            .filter(move |row| row.bp_id == bp_id)
    }

    pub fn battle_pass_bonus(&self, bp_id: i32, level: i32) -> Option<&BpLvBonus> {
        self.battle_pass_bonuses(bp_id)
            .find(|row| row.level == level)
    }

    pub fn battle_pass_tasks(&self, bp_id: i32) -> impl Iterator<Item = &BpTask> {
        self.bp_task.iter().filter(move |row| row.bp_id == bp_id)
    }
}
