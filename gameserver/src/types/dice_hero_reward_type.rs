#[derive(Clone, Copy)]
pub enum DiceHeroRewardType {
    Hero = 1,
    SkillCard = 2,
    Relic = 3,
}

impl DiceHeroRewardType {
    pub const fn id(self) -> i32 {
        self as i32
    }
}
