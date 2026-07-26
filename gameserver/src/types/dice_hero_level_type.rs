#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DiceHeroLevelType {
    Story = 1,
    Fight = 2,
}

impl DiceHeroLevelType {
    pub const fn from_id(id: i32) -> Option<Self> {
        match id {
            1 => Some(Self::Story),
            2 => Some(Self::Fight),
            _ => None,
        }
    }
}
