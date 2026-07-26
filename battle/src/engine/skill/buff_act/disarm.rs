use crate::engine::skill::effect::catalog::SkillEffectTag;

pub(crate) fn blocks(effect_tag: i32, is_big_skill: bool) -> bool {
    !is_big_skill
        && matches!(
            effect_tag,
            tag if tag == SkillEffectTag::RealityDamage as i32
                || tag == SkillEffectTag::MentalDamage as i32
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_only_basic_attack_incantations() {
        assert!(blocks(SkillEffectTag::RealityDamage as i32, false));
        assert!(blocks(SkillEffectTag::MentalDamage as i32, false));
        assert!(!blocks(SkillEffectTag::Buff as i32, false));
        assert!(!blocks(SkillEffectTag::RealityDamage as i32, true));
    }
}
