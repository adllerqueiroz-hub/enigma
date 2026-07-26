use super::*;

pub(super) fn parse_up_heroes(value: &str) -> (Vec<i32>, Vec<i32>) {
    let mut parts = value.split('|');
    (
        parts.next().map(parse_ids).unwrap_or_default(),
        parts.next().map(parse_ids).unwrap_or_default(),
    )
}

pub(super) fn parse_ids(value: &str) -> Vec<i32> {
    value
        .split('#')
        .filter_map(|part| part.parse::<i32>().ok())
        .collect()
}

pub(super) fn parse_weighted(value: &str) -> Vec<(i32, u32)> {
    value
        .split('|')
        .filter_map(|part| {
            let mut fields = part.split('#');
            Some((fields.next()?.parse().ok()?, fields.next()?.parse().ok()?))
        })
        .collect()
}

pub(super) fn choose_weighted(rng: &mut impl Rng, values: &[(i32, u32)]) -> i32 {
    let total = values.iter().map(|(_, weight)| *weight).sum::<u32>();
    let mut roll = rng.random_range(0..total);
    for (id, weight) in values {
        if roll < *weight {
            return *id;
        }
        roll -= *weight;
    }
    values[0].0
}
