#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DiceHeroGetRewardType {
    None = 0,
    All = 1,
    One = 2,
}

impl DiceHeroGetRewardType {
    pub const fn from_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Self::None),
            1 => Some(Self::All),
            2 => Some(Self::One),
            _ => None,
        }
    }
}
