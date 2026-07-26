pub const SOURCE_TARGET_CODE: i32 = 103;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TargetRequest {
    pub code: i32,
    pub raw: Vec<i32>,
}

impl TargetRequest {
    pub fn self_only() -> Self {
        Self {
            code: 0,
            raw: Vec::new(),
        }
    }
}

pub fn target_count(code: i32) -> i32 {
    config::try_get()
        .and_then(|db| db.ai_monster_target.get(code))
        .map(|row| row.target_number)
        .unwrap_or_default()
}

pub fn damage_target_count_kind(code: i32) -> i32 {
    match target_count(code) {
        1 => 1,
        count if count > 1 => 2,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_target_count_distinguishes_single_and_mass_attacks() {
        crate::test_support::init_config();

        assert_eq!(damage_target_count_kind(1), 1);
        assert_eq!(damage_target_count_kind(201), 2);
        assert_eq!(damage_target_count_kind(202), 2);
        assert_eq!(damage_target_count_kind(i32::MAX), 0);
    }
}
