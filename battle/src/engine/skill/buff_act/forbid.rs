use crate::engine::skill::effect::catalog::SkillEffectTag;

pub(crate) fn blocks(effect_tag: i32, is_big_skill: bool) -> bool {
    !is_big_skill
        && matches!(
            effect_tag,
            tag if tag == SkillEffectTag::Debuff as i32
                || tag == SkillEffectTag::Buff as i32
                || tag == SkillEffectTag::CounterSpell as i32
                || tag == SkillEffectTag::Heal as i32
        )
}
