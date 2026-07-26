#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuffStatus {
    Unknown = 0,
    StatsUp = 1,
    StatsDown = 2,
    Counter = 3,
    Control = 4,
    PositiveStatus = 5,
    NegativeStatus = 6,
    Shield = 7,
    Special = 8,
    Equipment = 13,
    Channel = 14,
}

impl BuffStatus {
    pub fn from_id(id: i32) -> Self {
        match id {
            1 => Self::StatsUp,
            2 => Self::StatsDown,
            3 => Self::Counter,
            4 => Self::Control,
            5 => Self::PositiveStatus,
            6 => Self::NegativeStatus,
            7 => Self::Shield,
            8 => Self::Special,
            13 => Self::Equipment,
            14 => Self::Channel,
            _ => Self::Unknown,
        }
    }

    pub fn is_good(self) -> bool {
        matches!(self, Self::StatsUp | Self::Counter | Self::PositiveStatus)
    }

    pub fn is_bad(self) -> bool {
        matches!(self, Self::StatsDown | Self::Control | Self::NegativeStatus)
    }
}

#[cfg(test)]
mod tests {
    use super::BuffStatus;

    #[test]
    fn good_and_bad_status_sets_match_fight_enum() {
        assert!(BuffStatus::StatsUp.is_good());
        assert!(BuffStatus::Counter.is_good());
        assert!(BuffStatus::PositiveStatus.is_good());
        assert!(BuffStatus::StatsDown.is_bad());
        assert!(BuffStatus::Control.is_bad());
        assert!(BuffStatus::NegativeStatus.is_bad());
        assert!(!BuffStatus::Shield.is_good());
        assert!(!BuffStatus::Shield.is_bad());
        assert!(!BuffStatus::Special.is_good());
        assert!(!BuffStatus::Special.is_bad());
    }
}
