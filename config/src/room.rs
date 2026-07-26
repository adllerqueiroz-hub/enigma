use crate::{GameDB, building_bonus::BuildingBonus, room_level::RoomLevel};

impl GameDB {
    pub fn room_level(&self, level: i32) -> Option<&RoomLevel> {
        self.room_level.iter().find(|row| row.level == level)
    }

    pub fn initial_room_level(&self) -> i32 {
        self.room_level
            .iter()
            .map(|row| row.level)
            .min()
            .unwrap_or_default()
    }

    pub fn building_bonus(&self, degree: i32) -> Option<&BuildingBonus> {
        self.building_bonus
            .iter()
            .filter(|row| row.build_degree <= degree)
            .max_by_key(|row| row.build_degree)
    }
}
