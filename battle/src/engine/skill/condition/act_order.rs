use super::parse::ParsedConditionKind;

pub fn order(_: i32, _: &str, args: &[String]) -> Option<ParsedConditionKind> {
    let [order] = args else { return None };
    Some(ParsedConditionKind::ActionOrder(order.parse().ok()?))
}

pub fn range(_: i32, _: &str, args: &[String]) -> Option<ParsedConditionKind> {
    let [start, count] = args else { return None };
    Some(ParsedConditionKind::ActionOrderRange {
        start: start.parse().ok()?,
        count: count.parse().ok()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_team_incantation_order_window() {
        assert_eq!(
            range(718212, "ActOrderRange", &["5".into(), "3".into()]),
            Some(ParsedConditionKind::ActionOrderRange { start: 5, count: 3 })
        );
    }
}
