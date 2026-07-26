use super::parse::{ParsedConditionKind, parse_i32_args};

pub fn present(_: i32, _: &str, args: &[String]) -> Option<ParsedConditionKind> {
    Some(ParsedConditionKind::InMagicCircleId(parse_i32_args(args)?))
}

pub fn absent(_: i32, _: &str, args: &[String]) -> Option<ParsedConditionKind> {
    Some(ParsedConditionKind::NotInMagicCircleId(parse_i32_args(
        args,
    )?))
}

pub fn added(_: i32, _: &str, args: &[String]) -> Option<ParsedConditionKind> {
    Some(ParsedConditionKind::AddedMagicCircle(parse_i32_args(args)?))
}

pub fn removed(_: i32, _: &str, args: &[String]) -> Option<ParsedConditionKind> {
    Some(ParsedConditionKind::RemovedMagicCircle(parse_i32_args(
        args,
    )?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_magic_circle_conditions() {
        let present = present(542208, "InMagicCircleId", &["10,11".into()]).unwrap();
        let added = added(711039, "AddMagicCircle", &["10".into()]).unwrap();
        let removed = removed(712040, "RemoveMagicCircle", &["10".into()]).unwrap();

        assert!(matches!(present, ParsedConditionKind::InMagicCircleId(_)));
        assert!(matches!(added, ParsedConditionKind::AddedMagicCircle(_)));
        assert!(matches!(
            removed,
            ParsedConditionKind::RemovedMagicCircle(_)
        ));
    }
}
