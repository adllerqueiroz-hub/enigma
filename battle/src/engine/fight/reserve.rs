#[derive(Debug, Clone, PartialEq)]
pub struct Promotion {
    pub defeated_uid: i64,
    pub entering_uid: i64,
    pub position: i32,
    pub team_type: i32,
    pub entering: sonettobuf::FightEntityInfo,
}
