pub fn supports(args: &[i32]) -> bool {
    let Some((&selection_count, skills)) = args.split_last() else {
        return false;
    };
    selection_count > 0
        && selection_count as usize <= skills.len()
        && skills.len() < i32::BITS as usize
        && skills.iter().all(|skill_id| *skill_id > 0)
}

pub fn initial_params(args: &[i32]) -> Option<Vec<i32>> {
    let (&selection_count, skills) = args.split_last()?;
    supports(args).then(|| {
        [0, selection_count]
            .into_iter()
            .chain(skills.iter().copied())
            .collect()
    })
}

pub fn select(args: &[i32], packed: i32) -> Option<(Vec<i32>, Vec<i32>)> {
    let (&selection_count, skills) = args.split_last()?;
    if !supports(args) || packed <= 0 {
        return None;
    }
    let packed = u32::try_from(packed).ok()?;
    let allowed = (1_u32 << skills.len()) - 1;
    if packed & !allowed != 0 || packed.count_ones() != selection_count as u32 {
        return None;
    }
    let selected = skills
        .iter()
        .enumerate()
        .filter_map(|(index, skill_id)| ((packed >> index) & 1 == 1).then_some(*skill_id))
        .collect();
    let params = [packed as i32, selection_count]
        .into_iter()
        .chain(skills.iter().copied())
        .collect();
    Some((selected, params))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitmask_selects_only_configured_precast_options() {
        let args = [101, 102, 103, 1];

        assert_eq!(initial_params(&args), Some(vec![0, 1, 101, 102, 103]));
        assert_eq!(
            select(&args, 4),
            Some((vec![103], vec![4, 1, 101, 102, 103]))
        );
        assert!(select(&args, 3).is_none());
        assert!(select(&args, 8).is_none());
    }
}
