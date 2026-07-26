use crate::engine::entity::attr::AttrId;

pub fn attribute_delta(values: &[i32], layer: i32, attr_id: AttrId) -> i32 {
    let Some((_, attributes)) = values.split_first() else {
        return 0;
    };
    let layer = layer.max(1);
    attributes
        .chunks_exact(3)
        .filter_map(|values| {
            let [configured_attr, base, per_extra] = values else {
                return None;
            };
            (AttrId::from_raw(*configured_attr) == Some(attr_id))
                .then_some(base.saturating_add(per_extra.saturating_mul(layer - 1)))
        })
        .fold(0, i32::saturating_add)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn firebud_applies_base_and_per_extra_stack_attributes() {
        let values = [922, 202, -150, -15, 204, -300, -30];

        assert_eq!(
            attribute_delta(&values, 3, AttrId::CriticalResistRate),
            -180
        );
        assert_eq!(attribute_delta(&values, 3, AttrId::CriticalDef), -360);
    }
}
