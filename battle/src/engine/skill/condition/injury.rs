use super::parse::{ParsedConditionKind, first_i32, parse_i32};

pub fn round_count(_: i32, _: &str, args: &[String]) -> Option<ParsedConditionKind> {
    Some(ParsedConditionKind::TeamInjuryCountRound {
        max_count: first_i32(args)?,
    })
}

pub fn teammate_count(_: i32, _: &str, args: &[String]) -> Option<ParsedConditionKind> {
    teammate_count_with_persistence(args, false)
}

pub fn persistent_teammate_count(_: i32, _: &str, args: &[String]) -> Option<ParsedConditionKind> {
    teammate_count_with_persistence(args, true)
}

fn teammate_count_with_persistence(
    args: &[String],
    persistent: bool,
) -> Option<ParsedConditionKind> {
    Some(ParsedConditionKind::TeammateInjuryCount {
        persistent,
        threshold: args.first().and_then(|arg| parse_i32(arg)).unwrap_or(1),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_injury_count_keeps_its_configured_cap() {
        assert_eq!(
            round_count(578, "TeamInjuryCountRound", &["20".into()]),
            Some(ParsedConditionKind::TeamInjuryCountRound { max_count: 20 })
        );
    }
}
