use crate::engine::skill::condition::parse::{ParsedConditionKind, parse_fixed};

pub fn ex_point(_: i32, _: &str, args: &[String]) -> Option<ParsedConditionKind> {
    let [compare_code, threshold] = parse_fixed(args)?;
    Some(ParsedConditionKind::ConduitExPoint {
        compare_code,
        threshold,
    })
}

pub fn selected_group(_: i32, _: &str, args: &[String]) -> Option<ParsedConditionKind> {
    let [group] = parse_fixed(args)?;
    (group > 0).then_some(ParsedConditionKind::ConduitSkillGroup { group })
}
