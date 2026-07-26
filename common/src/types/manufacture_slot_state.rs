#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManufactureSlotState {
    None,
    Running,
    Wait,
    Stop,
    Complete,
}

impl ManufactureSlotState {
    pub fn id(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Running => 1,
            Self::Wait => 2,
            Self::Stop => 3,
            Self::Complete => 4,
        }
    }
}
