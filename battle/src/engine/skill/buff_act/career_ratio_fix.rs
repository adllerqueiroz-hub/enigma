use crate::engine::manager::buff::ActiveBuffFeature;

pub fn bonus(feature: &ActiveBuffFeature) -> i32 {
    values_bonus(&feature.values, feature.amount)
}

pub fn supports(args: &[i32]) -> bool {
    matches!(args, [value] if *value != 0)
}

fn values_bonus(values: &[i32], amount: i32) -> i32 {
    match values {
        [_, value] => value * amount,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pragmatist_adds_twenty_percent_to_stronger_afflatus() {
        assert_eq!(values_bonus(&[765, 200], 1), 200);
    }
}
