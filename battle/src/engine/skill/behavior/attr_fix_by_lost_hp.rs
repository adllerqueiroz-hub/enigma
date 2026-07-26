use crate::engine::{
    entity::attr::AttrId,
    manager::BattleManagers,
    skill::{
        behavior::{BehaviorOpContext, classify::BehaviorKind, registry::BehaviorHandler},
        effect::ParsedBehavior,
        rule::output::RuleOp,
    },
};

pub(super) struct Handler;

impl BehaviorHandler for Handler {
    const VALIDATES_ARGUMENTS: bool = true;

    fn supports(behavior: &ParsedBehavior) -> bool {
        matches!(
            behavior.args.as_slice(),
            [bucket_size, raw_attr, _per_bucket, max_buckets]
                if *bucket_size > 0
                    && AttrId::from_raw(*raw_attr).is_some()
                    && *max_buckets >= 0
        )
    }

    fn emit_ops(context: BehaviorOpContext<'_>, behavior: &ParsedBehavior) -> Option<Vec<RuleOp>> {
        let (attr_id, delta) = resolve_for_context(&context, behavior)?;
        if context.active_skill_id != 0 && delta != 0 {
            context.modifiers.attack_attributes.push((attr_id, delta));
        }
        Some(Vec::new())
    }
}

pub fn resolve(
    source_uid: i64,
    managers: &BattleManagers,
    behavior: &ParsedBehavior,
) -> Option<(AttrId, i32)> {
    let hp = managers.hp.get(source_uid);
    resolve_at(hp.current, hp.max, behavior)
}

fn resolve_for_context(
    context: &BehaviorOpContext<'_>,
    behavior: &ParsedBehavior,
) -> Option<(AttrId, i32)> {
    let hp = context.managers.hp.get(context.source_uid);
    let action_start = (context.active_skill_id != 0)
        .then(|| {
            context
                .managers
                .hp
                .action_start(context.source_uid, context.active_skill_id)
        })
        .flatten();
    let snapshot = action_start.unwrap_or(hp);
    let resolved = resolve_at(snapshot.current, snapshot.max, behavior)?;
    if crate::engine::diagnostics::enabled(crate::engine::diagnostics::TraceArea::Damage) {
        eprintln!(
            "missing-hp modifier skill={} source={} snapshot={}/{} origin={} attr={:?} delta={}",
            context.active_skill_id,
            context.source_uid,
            snapshot.current,
            snapshot.max,
            if action_start.is_some() {
                "action-start"
            } else {
                "current"
            },
            resolved.0,
            resolved.1,
        );
    }
    Some(resolved)
}

fn resolve_at(current_hp: i32, max_hp: i32, behavior: &ParsedBehavior) -> Option<(AttrId, i32)> {
    if behavior.spec.kind != BehaviorKind::AttrFixByLoseHp {
        return None;
    }
    let [bucket_size, attr_id, per_bucket, max_buckets] = behavior.args.as_slice() else {
        return None;
    };
    if max_hp <= 0 || *bucket_size <= 0 {
        return None;
    }
    let missing = (max_hp - current_hp).max(0) * 1000 / max_hp;
    let buckets = (missing / *bucket_size).min((*max_buckets).max(0));
    Some((AttrId::from_raw(*attr_id)?, per_bucket * buckets))
}

#[cfg(test)]
mod tests {
    use sonettobuf::{Fight, FightEntityInfo, FightTeam, HeroAttribute};

    use super::*;

    #[test]
    fn scales_by_missing_hp_buckets_and_caps() {
        let managers = BattleManagers::seeded(&Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(10),
                    current_hp: Some(1_000),
                    attr: Some(HeroAttribute {
                        hp: Some(10_000),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        });
        let behavior = ParsedBehavior::new(
            60033,
            "AttrFixByLoseHp",
            vec![100, AttrId::DmgBonus as i32, 75, 8],
        );

        assert_eq!(
            resolve(10, &managers, &behavior),
            Some((AttrId::DmgBonus, 600))
        );
    }
}
