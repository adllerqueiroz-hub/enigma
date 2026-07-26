use crate::engine::{
    event::payload::BattleEvent,
    manager::{BattleManagers, buff::ActiveBuffFeature},
    mechanic::lingering_glow,
    skill::{
        buff_act::{BuffActRuleOp, registry::BuffActKind},
        effect::SkillEffectCatalog,
        rule::{SetupStage, output::RuleOp},
        subscriber::BuffActSubscriber,
    },
};

pub fn setup_rule_ops(
    managers: &BattleManagers,
    catalog: &SkillEffectCatalog,
    feature: &ActiveBuffFeature,
    stage: SetupStage,
) -> Option<Vec<RuleOp>> {
    if !super::is_kind(feature, BuffActKind::HeatScaleTag)
        || !super::is_primary_team_feature(managers, feature, BuffActKind::HeatScaleTag)
    {
        return Some(Vec::new());
    }
    match stage {
        SetupStage::BattleStart => Some(
            lingering_glow::enable_rule_ops(
                &managers.gauge,
                &managers.buff.active_features(&managers.hp),
                catalog,
            )
            .into_iter()
            .filter(|enable| enable.create.team == feature.team_type)
            .map(|enable| enable.output)
            .collect(),
        ),
        SetupStage::BuffGate | SetupStage::RoundStart => {
            Some(lingering_glow::round_start_attribute_rule_ops_for_team(
                managers,
                catalog,
                feature.team_type,
            ))
        }
        _ => Some(Vec::new()),
    }
}

pub fn rule_ops(
    managers: &BattleManagers,
    subscriber: &BuffActSubscriber,
    event: &BattleEvent,
) -> Option<Vec<BuffActRuleOp>> {
    if !super::subscriber_is_kind(subscriber, BuffActKind::HeatScaleTag)
        || !super::is_primary_team_subscriber(managers, subscriber, BuffActKind::HeatScaleTag)
    {
        return Some(Vec::new());
    }
    if let BattleEvent::GaugeChanged(change) = event {
        if change.key != lingering_glow::key(subscriber.team_type) || change.applied_delta <= 0 {
            return Some(Vec::new());
        }
        let features = managers.buff.active_features(&managers.hp);
        let Some(counter) =
            lingering_glow::visible_counter_info(&managers.gauge, &features, subscriber.team_type)
        else {
            return Some(Vec::new());
        };
        return Some(vec![BuffActRuleOp::causing(RuleOp::BuffActInfoMarker(
            crate::engine::manager::buff::BuffActInfoMarkerResult {
                target_uid: counter.owner_uid,
                buff_uid: counter.buff_uid,
                act_id: counter.act_id,
                params: vec![counter.current],
                str_param: Some(String::new()),
                team_type: 0,
            },
        ))]);
    }
    let change = match event {
        BattleEvent::BuffAdded(change) | BattleEvent::BuffChanged(change)
            if change.after_amount > change.before_amount =>
        {
            change
        }
        _ => return Some(Vec::new()),
    };
    if managers.buff.team_type(change.source_uid) != Some(subscriber.team_type) {
        return Some(Vec::new());
    }
    let added_layers = change.after_amount - change.before_amount;
    let Some(target_team) = managers.buff.team_type(change.target_uid) else {
        return Some(Vec::new());
    };
    let alive_enemies = managers.entity.alive_combatants(target_team, &managers.hp);
    let Some(alive_enemy_index) = alive_enemies
        .iter()
        .position(|uid| *uid == change.target_uid)
    else {
        return Some(Vec::new());
    };
    let features = managers.buff.active_features(&managers.hp);
    let ops = lingering_glow::burn_or_halo_rule_op(
        &managers.gauge,
        &features,
        crate::engine::mechanic::heat_scale::BurnOrHaloAdded {
            source_team: subscriber.team_type,
            target_uid: change.target_uid,
            buff_uid: change.buff_uid,
            added_layers,
            alive_enemy_index,
            alive_enemy_count: alive_enemies.len(),
        },
    )
    .and_then(|input| match input.output {
        RuleOp::Command(crate::engine::skill::rule::output::BattleCommand::Gauge(command)) => {
            Some(lingering_glow::value_change_rule_ops(managers, command))
        }
        _ => None,
    })
    .unwrap_or_default()
    .into_iter()
    .map(BuffActRuleOp::causing)
    .collect();
    Some(ops)
}
