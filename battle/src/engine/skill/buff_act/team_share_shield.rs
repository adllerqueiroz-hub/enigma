use crate::engine::entity::attr::AttrId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TeamShareShieldRule {
    pub attribute: AttrId,
    pub max_rate: i32,
    counts: [i32; 4],
    rates: [i32; 4],
}

pub fn supports(args: &[i32]) -> bool {
    parse(args).is_some()
}

pub fn block_rate(args: &[i32], target_count: usize) -> Option<i32> {
    let rule = parse(args)?;
    let target_count = i32::try_from(target_count).ok()?;
    rule.counts
        .iter()
        .position(|count| *count == target_count)
        .map(|index| rule.rates[index])
}

pub fn parse(args: &[i32]) -> Option<TeamShareShieldRule> {
    let [
        raw_attr,
        amount_rate,
        max_rate,
        one,
        two,
        three,
        four,
        rate_one,
        rate_two,
        rate_three,
        rate_four,
    ] = args
    else {
        return None;
    };
    let counts = [*one, *two, *three, *four];
    let rates = [*rate_one, *rate_two, *rate_three, *rate_four];
    let attribute = AttrId::from_raw(*raw_attr)?;
    (*amount_rate > 0
        && *max_rate > 0
        && counts == [1, 2, 3, 4]
        && rates.iter().all(|rate| *rate > 0))
    .then_some(TeamShareShieldRule {
        attribute,
        max_rate: *max_rate,
        counts,
        rates,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASTION: [i32; 11] = [102, 2_800, 12_500, 1, 2, 3, 4, 1_000, 1_200, 1_500, 1_800];

    #[test]
    fn parses_the_configured_target_count_rates() {
        assert!(supports(&BASTION));
        assert_eq!(block_rate(&BASTION, 1), Some(1_000));
        assert_eq!(block_rate(&BASTION, 2), Some(1_200));
        assert_eq!(block_rate(&BASTION, 4), Some(1_800));
        assert_eq!(block_rate(&BASTION, 5), None);
    }
}
