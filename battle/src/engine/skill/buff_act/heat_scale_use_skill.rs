use crate::engine::{
    manager::BattleManagers,
    mechanic::lingering_glow,
    skill::{effect::SkillEffectCatalog, subscriber::BuffActSubscriber},
};

pub fn rule_ops(
    managers: &BattleManagers,
    catalog: &SkillEffectCatalog,
    subscriber: &BuffActSubscriber,
) -> Vec<super::BuffActRuleOp> {
    lingering_glow::ready_cast_rule_ops(
        &managers.gauge,
        &managers.buff,
        &managers.emanation,
        catalog,
        subscriber,
    )
    .map(|cast| {
        cast.outputs
            .into_iter()
            .map(super::BuffActRuleOp::subscriber_from_owner)
            .collect()
    })
    .unwrap_or_default()
}
