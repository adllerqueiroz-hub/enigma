use crate::engine::{
    entity::attr::AttrId,
    manager::{
        BattleManagers,
        buff::ActiveBuffFeature,
        hp::{HpCommand, MaxHpAdjust},
    },
    skill::{
        buff_act::{self, registry::BuffActKind},
        rule::output::{BattleCommand, RuleOp},
    },
};

pub fn max_hp_rule_op(
    managers: &BattleManagers,
    feature: &ActiveBuffFeature,
    amount_delta: i32,
) -> Option<RuleOp> {
    if !buff_act::is_kind(feature, BuffActKind::EachChangeAttr) || amount_delta == 0 {
        return None;
    }
    let [_, raw_attr, flat, source_attr, permille] = feature.values.as_slice() else {
        return None;
    };
    if AttrId::from_raw(*raw_attr) != Some(AttrId::Hp)
        || AttrId::from_raw(*source_attr) != Some(AttrId::Hp)
    {
        return None;
    }
    let source_uid = if feature.source_uid != 0 {
        feature.source_uid
    } else {
        feature.owner_uid
    };
    let delta = ((i128::from(managers.hp.max(source_uid)) * i128::from(*permille) / 1000
        + i128::from(*flat))
        * i128::from(amount_delta))
    .clamp(i128::from(i32::MIN), i128::from(i32::MAX)) as i32;
    (delta != 0).then_some(RuleOp::Command(BattleCommand::Hp(HpCommand::AdjustMax(
        MaxHpAdjust {
            origin: buff_act::feature_command_origin(feature)?,
            source_uid,
            target_uid: feature.owner_uid,
            delta,
        },
    ))))
}

pub fn transaction_rule_ops(
    managers: &BattleManagers,
    event: &crate::engine::event::payload::BattleEvent,
) -> Vec<(ActiveBuffFeature, RuleOp)> {
    super::attribute_transaction_rule_ops(
        managers,
        event,
        BuffActKind::EachChangeAttr,
        max_hp_rule_op,
    )
}
