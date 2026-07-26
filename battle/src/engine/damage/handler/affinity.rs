use crate::engine::skill::target::TargetEntity;

pub(super) fn regular_multiplier(source_bonus: i32, target_reduction: i32) -> i32 {
    (1000 + source_bonus - target_reduction).max(300)
}

pub(super) fn critical_technique_bonus(
    entity: &TargetEntity,
    target_level: i32,
    fight_const_id: i32,
) -> i32 {
    let Some(db) = config::try_get() else {
        return 0;
    };
    let value = |id| {
        db.fight_const
            .get(id)
            .and_then(|row| row.value.parse::<i32>().ok())
            .unwrap_or_default()
    };
    technique_bonus(
        entity.technic,
        target_level,
        value(fight_const_id),
        value(13),
        value(14),
    )
}

pub(super) fn technique_bonus(
    technic: i32,
    target_level: i32,
    ratio: i32,
    correct: i32,
    level_ratio: i32,
) -> i32 {
    let denominator = correct + target_level * level_ratio;
    if denominator <= 0 {
        0
    } else {
        technic * ratio / denominator
    }
}

pub(crate) fn restrains(source: i32, target: i32) -> bool {
    career_multiplier(source, target) > 1000
}

pub(crate) fn restrains_target(source: i32, target: &TargetEntity) -> bool {
    target.weak_careers.contains(&source) || restrains(source, target.career)
}

pub(super) fn career_multiplier_against(source: i32, target: &TargetEntity) -> i32 {
    if target.weak_careers.contains(&source) {
        strongest_career_multiplier(source)
    } else {
        career_multiplier(source, target.career)
    }
}

pub(super) fn career_multiplier(source: i32, target: i32) -> i32 {
    let Some(row) = config::try_get().and_then(|db| db.fight_effect.get(source)) else {
        return 1000;
    };
    match target {
        1 => row.career1,
        2 => row.career2,
        3 => row.career3,
        4 => row.career4,
        5 => row.career5,
        6 => row.career6,
        7 => row.career7,
        8 => row.career8,
        _ => 1000,
    }
}

pub(super) fn strongest_career_multiplier(source: i32) -> i32 {
    let Some(row) = config::try_get().and_then(|db| db.fight_effect.get(source)) else {
        return 1000;
    };
    [
        row.career1,
        row.career2,
        row.career3,
        row.career4,
        row.career5,
        row.career6,
        row.career7,
        row.career8,
    ]
    .into_iter()
    .max()
    .unwrap_or(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_weakness_uses_the_standard_stronger_afflatus_multiplier() {
        crate::test_support::init_config();
        let target = TargetEntity::from_fight_entity(&sonettobuf::FightEntityInfo {
            uid: Some(-1),
            current_hp: Some(1),
            career: Some(8),
            weak_careers: vec![1, 2],
            ..Default::default()
        })
        .unwrap();

        assert!(restrains_target(1, &target));
        assert_eq!(career_multiplier_against(1, &target), 1300);
        assert!(!restrains_target(3, &target));
        assert_eq!(career_multiplier_against(3, &target), 1000);
    }
}
