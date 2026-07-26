use super::registry::BuffActKind;

pub fn supports(args: &[i32]) -> bool {
    match args {
        [buff_id] => *buff_id > 0,
        [buff_id, layer] => *buff_id > 0 && *layer > 0,
        _ => false,
    }
}

pub fn referenced_buff(args: &[i32]) -> Option<i32> {
    supports(args).then(|| args[0])
}

pub fn linked_buff_id(kind: Option<BuffActKind>, values: &[i32]) -> Option<i32> {
    (kind == Some(BuffActKind::AddBuffToEnter)).then(|| referenced_buff(values.get(1..)?))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_a_positive_link_and_optional_positive_layer() {
        assert_eq!(referenced_buff(&[31280120]), Some(31280120));
        assert_eq!(referenced_buff(&[31280120, 2]), Some(31280120));
        assert_eq!(referenced_buff(&[31280120, 0]), None);
        assert_eq!(referenced_buff(&[]), None);
    }
}
