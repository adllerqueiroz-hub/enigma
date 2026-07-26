#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CardOpType {
    MoveCard = 1,
    PlayCard = 2,
    MoveUniversal = 3,
    AssistBoss = 4,
    Season2ChangeHero = 5,
    PlayerFinisherSkill = 6,
    BloodPool = 7,
    SimulateDissolveCard = -99,
    Rouge2Music = -100,
    Unknown = 0,
}

impl CardOpType {
    pub fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::MoveCard,
            2 => Self::PlayCard,
            3 => Self::MoveUniversal,
            4 => Self::AssistBoss,
            5 => Self::Season2ChangeHero,
            6 => Self::PlayerFinisherSkill,
            7 => Self::BloodPool,
            -99 => Self::SimulateDissolveCard,
            -100 => Self::Rouge2Music,
            _ => Self::Unknown,
        }
    }

    pub const fn id(self) -> i32 {
        self as i32
    }
}

impl From<i32> for CardOpType {
    fn from(value: i32) -> Self {
        Self::from_raw(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_wire_card_op_values() {
        assert_eq!(CardOpType::from(1), CardOpType::MoveCard);
        assert_eq!(CardOpType::from(2), CardOpType::PlayCard);
        assert_eq!(CardOpType::from(7), CardOpType::BloodPool);
        assert_eq!(CardOpType::from(-99), CardOpType::SimulateDissolveCard);
        assert_eq!(CardOpType::from(12345), CardOpType::Unknown);
    }
}
