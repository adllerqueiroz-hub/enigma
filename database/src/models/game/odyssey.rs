use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct OdysseyState {
    pub user_id: i64,
    pub exp: i32,
    pub level: i32,
    pub params: String,
    pub curr_element_id: i32,
    pub talent_point: i32,
    pub cassandra_tree: String,
    pub next_mercenary_refresh: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct OdysseyMapState {
    pub user_id: i64,
    pub map_id: i32,
    pub explore_value: i32,
    pub updated_at: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct OdysseyElementState {
    pub user_id: i64,
    pub element_id: i32,
    pub status: i32,
    pub updated_at: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct OdysseyTalentState {
    pub user_id: i64,
    pub node_id: i32,
    pub level: i32,
    pub consume: i32,
    pub updated_at: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct OdysseyItemState {
    pub user_id: i64,
    pub uid: i32,
    pub item_id: i32,
    pub count: i32,
    pub new_flag: bool,
    pub updated_at: i64,
}
