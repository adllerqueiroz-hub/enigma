#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManufactureSlotOperation {
    Add,
    Cancel,
    MoveTop,
    MoveBottom,
}

impl ManufactureSlotOperation {
    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Self::Add),
            1 => Some(Self::Cancel),
            4 => Some(Self::MoveTop),
            5 => Some(Self::MoveBottom),
            _ => None,
        }
    }
}
