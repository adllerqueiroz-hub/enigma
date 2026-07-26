use super::*;

impl EffectPacket {
    pub fn emitter_create() -> ActEffect {
        let attr = HeroAttribute {
            hp: Some(0),
            attack: Some(0),
            defense: Some(0),
            mdefense: Some(0),
            technic: Some(0),
            multi_hp_idx: Some(0),
            multi_hp_num: Some(0),
        };
        ActEffect {
            target_id: Some(0),
            effect_type: Some(EffectType::Emittercreate as i32),
            effect_num: Some(1),
            entity: Some(FightEntityInfo {
                uid: Some(emitter::UID),
                model_id: Some(0),
                skin: Some(0),
                position: Some(0),
                entity_type: Some(6),
                user_id: Some(0),
                ex_point: Some(0),
                level: Some(0),
                current_hp: Some(1),
                attr: Some(attr),
                ex_skill: Some(0),
                shield_value: Some(0),
                expoint_max_add: Some(0),
                buff_harm_statistic: Some(0),
                equip_uid: Some(0),
                trial_equip: Some(Default::default()),
                ex_skill_level: Some(0),
                base_attr: Some(attr),
                ex_skill_point_change: Some(0),
                team_type: Some(1),
                enhance_info_box: Some(EnhanceInfoBox {
                    uid: Some(emitter::UID),
                    ..Default::default()
                }),
                trial_id: Some(0),
                career: Some(0),
                status: Some(0),
                guard: Some(-1),
                sub_cd: Some(0),
                ex_point_type: Some(0),
                destiny_stone: Some(0),
                destiny_rank: Some(0),
                custom_unit_id: Some(0),
                ..Default::default()
            }),
            config_effect: Some(0),
            buff_act_id: Some(0),
            reserve_id: Some(0),
            team_type: Some(0),
            effect_num1: Some(0),
            emitter_info: Some(EmitterInfo { energy: Some(0) }),
            ..Default::default()
        }
    }

    pub fn emitter_energy_change(delta: i32) -> ActEffect {
        ActEffect {
            target_id: Some(crate::engine::manager::emitter::UID),
            effect_type: Some(EffectType::Emitterenergychange as i32),
            effect_num: Some(1),
            config_effect: Some(0),
            buff_act_id: Some(0),
            reserve_id: Some(0),
            team_type: Some(0),
            effect_num1: Some(delta),
            ..Default::default()
        }
    }

    pub fn emitter_skill_end() -> ActEffect {
        ActEffect {
            target_id: Some(crate::engine::manager::emitter::UID),
            effect_type: Some(EffectType::Emitterskillend as i32),
            effect_num: Some(1),
            config_effect: Some(0),
            buff_act_id: Some(0),
            reserve_id: Some(0),
            team_type: Some(0),
            effect_num1: Some(0),
            ..Default::default()
        }
    }

    pub fn emitter_attack_marker(
        attack: crate::engine::manager::emitter::EmitterAttack,
    ) -> ActEffect {
        ActEffect {
            target_id: Some(0),
            effect_type: Some(EffectType::Emitterfightnotify as i32),
            effect_num: Some(0),
            config_effect: Some(0),
            buff_act_id: Some(0),
            reserve_id: Some(0),
            reserve_str: Some(format!(
                "{{\"splitNum\":{},\"emitterAttackNum\":{},\"emitterAttackMaxNum\":{}}}",
                attack.split_count, attack.index, attack.max
            )),
            team_type: Some(0),
            effect_num1: Some(0),
            ..Default::default()
        }
    }

    pub fn round_end_deal(team_type: i32) -> Vec<ActEffect> {
        vec![
            ActEffect {
                effect_type: Some(EffectType::Roundend as i32),
                ..Default::default()
            },
            ActEffect {
                effect_type: Some(EffectType::Smallroundend as i32),
                effect_num: Some(team_type),
                ..Default::default()
            },
            ActEffect {
                effect_type: Some(EffectType::Dealcard2 as i32),
                ..Default::default()
            },
        ]
    }

    pub fn small_round_end(team_type: i32) -> ActEffect {
        ActEffect {
            target_id: Some(0),
            effect_type: Some(EffectType::Smallroundend as i32),
            effect_num: Some(team_type),
            ..Default::default()
        }
    }

    pub fn clear_universal_card() -> ActEffect {
        ActEffect {
            target_id: Some(0),
            effect_type: Some(EffectType::Clearuniversalcard as i32),
            effect_num: Some(0),
            team_type: Some(1),
            ..Default::default()
        }
    }

    pub fn change_round() -> ActEffect {
        ActEffect {
            target_id: Some(0),
            effect_type: Some(EffectType::Changeround as i32),
            effect_num: Some(0),
            ..Default::default()
        }
    }

    pub fn fight_counter(target_uid: i64, counter: i32, value: i32, team: i32) -> ActEffect {
        ActEffect {
            target_id: Some(target_uid),
            effect_type: Some(EffectType::Fightcounter as i32),
            effect_num: Some(counter),
            config_effect: Some(value),
            team_type: Some(team),
            effect_num1: Some(0),
            ..Default::default()
        }
    }

    pub fn fight_step(effects: Vec<ActEffect>) -> ActEffect {
        Self::nested_step(fight_step::ActType::Effect as i32, 0, 0, 0, 0, effects)
    }

    pub fn fake_fight_step(effects: Vec<ActEffect>) -> ActEffect {
        let mut effect = Self::fight_step(effects);
        if let Some(step) = effect.fight_step.as_mut() {
            step.fake_timeline = Some(true);
        }
        effect
    }
}
