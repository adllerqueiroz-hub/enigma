#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CritterSeatOperation {
    Change,
    Exchange,
}

impl CritterSeatOperation {
    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Self::Change),
            1 => Some(Self::Exchange),
            _ => None,
        }
    }
}
