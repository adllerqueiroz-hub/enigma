use crate::engine::{entity::attr::AttrId, skill::buff_act::registry};

pub fn supports(args: &[i32]) -> bool {
    matches!(args, [_, raw_attr, rate, ..]
        if AttrId::from_raw(*raw_attr).is_some() && *rate > 0)
}

pub fn configured_attr_rate(buff_id: i32) -> Option<(AttrId, i32)> {
    let row = config::try_get()?.skill_buff.get(buff_id)?;
    row.features.split('|').find_map(|feature| {
        let mut parts = feature.split('#');
        let act_id = parts.next()?.trim().parse::<i32>().ok()?;
        let act_type = &config::try_get()?.buff_act.get(act_id)?.r#type;
        if registry::kind(act_id, act_type) != Some(registry::BuffActKind::Shield) {
            return None;
        }
        let _mode = parts.next()?;
        let attr_id = AttrId::from_raw(parts.next()?.trim().parse().ok()?)?;
        let rate = parts.next()?.trim().parse().ok()?;
        Some((attr_id, rate))
    })
}
