use crate::engine::manager::buff::ActiveBuffFeature;

pub fn active(feature: &ActiveBuffFeature) -> bool {
    matches!(feature.values.as_slice(), [_])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pragmatist_forces_stronger_afflatus() {
        assert!(active(&ActiveBuffFeature {
            owner_uid: 1,
            source_uid: 1,
            buff_uid: 1,
            buff_id: 1,
            amount: 1,
            team_type: 1,
            owner_alive: true,
            act_type: String::new(),
            effect_time: 0,
            effect_condition: 0,
            raw: "764".to_owned(),
            values: vec![764],
        }));
    }
}
