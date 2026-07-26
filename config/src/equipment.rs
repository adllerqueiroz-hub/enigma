use crate::{GameDB, equip_break_cost::EquipBreakCost, equip_strengthen_cost::EquipStrengthenCost};

impl GameDB {
    pub fn equip_break_cost(&self, rare: i32, break_level: i32) -> Option<&EquipBreakCost> {
        self.equip_break_cost
            .iter()
            .find(|row| row.rare == rare && row.break_level == break_level)
    }

    pub fn equip_strengthen_cost(&self, rare: i32, level: i32) -> Option<&EquipStrengthenCost> {
        self.equip_strengthen_cost
            .iter()
            .find(|row| row.rare == rare && row.level == level)
    }

    pub fn max_equip_progression(&self, rare: i32) -> Option<&EquipBreakCost> {
        self.equip_break_cost
            .iter()
            .filter(|row| row.rare == rare)
            .max_by_key(|row| row.break_level)
    }
}
