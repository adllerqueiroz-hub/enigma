use crate::GameDB;

impl GameDB {
    pub fn has_scene_skin(&self, item_id: i32) -> bool {
        self.scene_switch.iter().any(|row| {
            (item_id == 0 && row.default_unlock == 1) || (item_id != 0 && row.item_id == item_id)
        })
    }

    pub fn has_ui_style_skin(&self, item_id: i32) -> bool {
        self.scene_ui.iter().any(|row| {
            (item_id == 0 && row.default_unlock == 1) || (item_id != 0 && row.item_id == item_id)
        })
    }
}
