use rand::Rng;
use sonettobuf::CardInfo;
use std::collections::HashMap;

pub fn draw_guaranteed_by_uid<R: Rng + ?Sized>(
    cards: &[CardInfo],
    required_uids: &[i64],
    count: usize,
    rng: &mut R,
) -> Vec<CardInfo> {
    if cards.is_empty() || count == 0 {
        return Vec::new();
    }

    let mut by_uid: HashMap<i64, Vec<&CardInfo>> = HashMap::new();
    for card in cards {
        by_uid
            .entry(card.uid.unwrap_or_default())
            .or_default()
            .push(card);
    }

    let mut out = Vec::with_capacity(count);
    for uid in required_uids {
        if out.len() >= count {
            break;
        }
        if let Some(options) = by_uid.get(uid) {
            out.push(pick_non_touching(options.iter().copied(), out.last(), rng).clone());
        }
    }

    while out.len() < count {
        out.push(pick_non_touching(cards.iter(), out.last(), rng).clone());
    }

    separate_adjacent_same_skill(&mut out);
    out
}

fn pick_non_touching<'a, R: Rng + ?Sized, I>(
    options: I,
    previous: Option<&CardInfo>,
    rng: &mut R,
) -> &'a CardInfo
where
    I: IntoIterator<Item = &'a CardInfo>,
{
    let previous_skill = previous.and_then(|card| card.skill_id);
    let options: Vec<_> = options.into_iter().collect();
    let choices: Vec<_> = options
        .iter()
        .copied()
        .filter(|card| card.skill_id != previous_skill)
        .collect();
    let choices = if choices.is_empty() { options } else { choices };
    choices[rng.random_range(0..choices.len())]
}

fn separate_adjacent_same_skill(cards: &mut [CardInfo]) {
    for i in 1..cards.len() {
        if cards[i].skill_id != cards[i - 1].skill_id {
            continue;
        }
        if let Some(replacement) =
            ((i + 1)..cards.len()).find(|index| cards[*index].skill_id != cards[i - 1].skill_id)
        {
            cards.swap(i, replacement);
        }
    }
}
