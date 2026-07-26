use crate::{GameDB, player_bg::PlayerBg, player_level::PlayerLevel};

impl GameDB {
    pub fn player_level(&self, level: i32) -> Option<&PlayerLevel> {
        self.player_level.iter().find(|row| row.level == level)
    }

    pub fn player_levels_between(&self, from: i32, to: i32) -> impl Iterator<Item = &PlayerLevel> {
        self.player_level
            .iter()
            .filter(move |row| (from..to).contains(&row.level))
    }

    pub fn player_background_by_item(&self, item_id: i32) -> Option<&PlayerBg> {
        self.player_bg.iter().find(|row| row.item == item_id)
    }
}
