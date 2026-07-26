use crate::engine::{
    manager::{
        BattleManagers,
        hp::{HpCommand, HpLoss, HurtDamageFromType, HurtInfoData},
    },
    skill::{
        rule::output::{BattleCommand, RuleOp},
        subscriber::BuffActSubscriber,
    },
};

pub fn matches(subscriber: &BuffActSubscriber) -> bool {
    super::subscriber_is_kind(
        subscriber,
        super::registry::BuffActKind::LostHpAddExtraBloodPoolValue,
    )
}

pub fn rule_ops(managers: &BattleManagers, subscriber: &BuffActSubscriber) -> Vec<RuleOp> {
    if !matches(subscriber) {
        return Vec::new();
    }
    let [_, rate, _, _, ..] = subscriber.args.as_slice() else {
        return Vec::new();
    };
    let amount = loss_amount(
        managers.hp.max(subscriber.owner_uid),
        *rate,
        subscriber.amount,
    );
    let Some(origin) = super::command_origin(subscriber) else {
        return Vec::new();
    };
    vec![RuleOp::Command(BattleCommand::Hp(HpCommand::Lose(
        HpLoss {
            origin,
            source_uid: subscriber.owner_uid,
            target_uid: subscriber.owner_uid,
            amount,
            config_effect: 0,
            hurt: Some(HurtInfoData {
                from_uid: subscriber.source_uid,
                is_crit: false,
                career_restraint: false,
                reduce_hp: 0,
                effect_id: 0,
                skill_id: 0,
                damage_from: HurtDamageFromType::Buff,
                buff_act_id: subscriber.key.definition.opcode,
                buff_uid: subscriber.buff_uid,
                hurt_effect_type: 0,
                display_amount: None,
            }),
        },
    )))]
}

pub fn observed_loss(
    managers: &BattleManagers,
    target_uid: i64,
    buff_uid: Option<i64>,
    actual_loss: i32,
) -> i32 {
    let Some(buff_uid) = buff_uid else {
        return actual_loss;
    };
    let extra_rate = managers
        .buff
        .active_features(&managers.hp)
        .into_iter()
        .find(|feature| {
            feature.owner_uid == target_uid
                && feature.buff_uid == buff_uid
                && super::is_kind(
                    feature,
                    super::registry::BuffActKind::LostHpAddExtraBloodPoolValue,
                )
        })
        .and_then(|feature| feature.values.get(4).copied())
        .unwrap_or_default();
    boosted_loss(actual_loss, extra_rate)
}

fn loss_amount(max_hp: i32, rate: i32, stacks: i32) -> i32 {
    max_hp.max(0) * rate.max(0) * stacks.max(1) / 1000
}

fn boosted_loss(loss: i32, extra_rate: i32) -> i32 {
    loss.max(0) * (1000 + extra_rate.max(0)) / 1000
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{event::kind::EventKind, skill::subscriber};
    use sonettobuf::{Fight, FightEntityInfo, FightTeam, HeroAttribute};

    #[test]
    fn stacked_loss_and_extra_bloodtithe_follow_the_buff_payload() {
        assert_eq!(loss_amount(17_513, 15, 4), 1_050);
        assert_eq!(boosted_loss(1_050, 500), 1_575);
    }

    #[test]
    fn configured_loss_keeps_actual_hp_and_bloodtithe_observation_separate() {
        crate::test_support::init_config();
        let fight = Fight {
            attacker: Some(FightTeam {
                entitys: vec![FightEntityInfo {
                    uid: Some(10),
                    team_type: Some(1),
                    current_hp: Some(10_000),
                    attr: Some(HeroAttribute {
                        hp: Some(10_000),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut managers = BattleManagers::seeded(&fight);
        managers.buff.add(&managers.hp, 10, 10, 31260121, 0);
        let subscriber = subscriber::for_active_buffs(&managers, EventKind::RoundStart)
            .into_iter()
            .find(matches)
            .unwrap();

        assert!(matches!(
            rule_ops(&managers, &subscriber).as_slice(),
            [RuleOp::Command(BattleCommand::Hp(HpCommand::Lose(
                HpLoss { amount: 150, .. }
            )))]
        ));
        assert_eq!(
            observed_loss(&managers, 10, Some(subscriber.buff_uid), 150),
            225
        );
    }
}
