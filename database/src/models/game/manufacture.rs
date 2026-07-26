use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct ManufactureSlot {
    pub user_id: i64,
    pub building_uid: i64,
    pub slot_id: i32,
    pub priority: i32,
    pub production_id: i32,
    pub slot_status: i32,
    pub inventory_count: i32,
    pub begin_time: i32,
    pub next_finish_time: i32,
    pub pause_time: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct FrozenItem {
    pub user_id: i64,
    pub material_id: i32,
    pub quantity: i32,
    pub time: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct RestCritter {
    pub building_uid: i64,
    pub rest_slot_id: i32,
    pub critter_uid: i64,
}
