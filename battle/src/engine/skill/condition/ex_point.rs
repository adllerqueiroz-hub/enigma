use super::parse::ExPointIncreaseScope;

pub fn increase_in_scope(
    event_target_uid: i64,
    source_uid: i64,
    scope: ExPointIncreaseScope,
    same_team: bool,
) -> bool {
    match scope {
        ExPointIncreaseScope::SelfOnly => event_target_uid == source_uid,
        ExPointIncreaseScope::OtherAlly => {
            event_target_uid != 0 && event_target_uid != source_uid && same_team
        }
    }
}

pub fn decrease_count(delta: i32, threshold: i32) -> i32 {
    if delta >= 0 {
        return 0;
    }
    delta
        .saturating_neg()
        .checked_div(threshold.max(1))
        .unwrap_or_default()
}

pub fn per_ex_point_count(threshold: i32, current: i32) -> Option<i32> {
    (current >= threshold).then_some(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_point_increase_scopes_stay_distinct() {
        assert!(increase_in_scope(
            10,
            10,
            ExPointIncreaseScope::SelfOnly,
            true
        ));
        assert!(!increase_in_scope(
            11,
            10,
            ExPointIncreaseScope::SelfOnly,
            true
        ));
        assert!(increase_in_scope(
            11,
            10,
            ExPointIncreaseScope::OtherAlly,
            true
        ));
        assert!(!increase_in_scope(
            12,
            10,
            ExPointIncreaseScope::OtherAlly,
            false
        ));
        assert!(!increase_in_scope(
            10,
            10,
            ExPointIncreaseScope::OtherAlly,
            true
        ));
    }

    #[test]
    fn per_ex_point_uses_the_parsed_threshold() {
        assert_eq!(per_ex_point_count(1, 3), Some(3));
        assert_eq!(per_ex_point_count(1, 0), None);
    }

    #[test]
    fn decrease_counts_each_spent_point() {
        assert_eq!(decrease_count(-3, 1), 3);
        assert_eq!(decrease_count(-3, 2), 1);
        assert_eq!(decrease_count(3, 1), 0);
    }
}
