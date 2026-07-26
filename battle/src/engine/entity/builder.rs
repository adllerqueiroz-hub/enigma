use database::{db::game::equipment::Equipment, models::game::heros::HeroData};
use sonettobuf::{EnhanceInfoBox, EquipRecord, FightEntityInfo, HeroAttribute, PowerInfo};

use super::{attr::Attr, destiny::Destiny, passive::Passive, skill::Skill};

pub struct EntityBuilder {
    hero_data: HeroData,
    equip: Option<Equipment>,
    position: i32,
    team_type: i32,
    is_sub: bool,
}

impl EntityBuilder {
    pub fn new(hero_data: HeroData, position: i32, team_type: i32, is_sub: bool) -> Self {
        Self {
            hero_data,
            equip: None,
            position,
            team_type,
            is_sub,
        }
    }

    pub fn with_equip(mut self, equip: Equipment) -> Self {
        self.equip = Some(equip);
        self
    }

    pub fn build(self) -> FightEntityInfo {
        let r = &self.hero_data.record;
        let destiny = Destiny::get(r.destiny_stone, r.destiny_rank);
        let attr = Attr::get(&self.hero_data, self.equip.as_ref());
        let (sg1, sg2) = Skill::get(&self.hero_data, self.is_sub, destiny.as_ref());
        let passives = Passive::get(
            &self.hero_data,
            self.equip.as_ref().map(|e| e.equip_id),
            destiny.as_ref(),
        );
        // Source attribution (Insight/Rank/Destiny/Psychube/Extra) is tracked
        // in `PassiveSkill` for downstream consumers; the wire format only
        // carries raw skill ids.
        let passive_skill_ids = passives.iter().map(|p| p.skill_id).collect();
        let equip_record = EquipRecord {
            equip_uid: self.equip.as_ref().map(|e| e.uid),
            equip_id: self.equip.as_ref().map(|e| e.equip_id),
            equip_lv: self.equip.as_ref().map(|e| e.level),
            refine_lv: self.equip.as_ref().map(|e| e.refine_lv),
        };

        FightEntityInfo {
            uid: Some(r.uid),
            model_id: Some(r.hero_id),
            skin: Some(r.skin),
            position: Some(self.position),
            entity_type: Some(1),
            user_id: Some(r.user_id),
            ex_point: Some(0),
            level: Some(r.level),
            current_hp: attr.hp,
            attr: Some(attr),
            base_attr: Some(attr),
            skill_group1: sg1,
            skill_group2: sg2,
            passive_skill: passive_skill_ids,
            ex_skill: Some(Skill::get_ex(&self.hero_data, destiny.as_ref())),
            shield_value: Some(0),
            expoint_max_add: Some(0),
            buff_harm_statistic: Some(0),
            equip_uid: Some(r.default_equip_uid),
            trial_equip: Some(EquipRecord::default()),
            ex_skill_level: Some(r.ex_skill_level),
            power_infos: Self::hero_power_infos(&self.hero_data),
            ex_skill_point_change: Some(0),
            team_type: Some(self.team_type),
            enhance_info_box: Some(EnhanceInfoBox {
                uid: Some(r.uid),
                can_upgrade_ids: vec![],
                upgraded_options: vec![],
            }),
            trial_id: Some(0),
            career: Some(Self::career(&self.hero_data)),
            status: Some(0),
            guard: Some(-1),
            sub_cd: Some(0),
            ex_point_type: Some(Self::ex_point_type(r.hero_id)),
            equips: vec![equip_record],
            destiny_stone: Some(r.destiny_stone),
            destiny_rank: Some(r.destiny_rank),
            custom_unit_id: Some(0),
            ..Default::default()
        }
    }

    pub fn player(user_id: i64, team_type: i32) -> FightEntityInfo {
        let uid = if team_type == 1 { 0 } else { -99999 };
        let attr = HeroAttribute {
            hp: Some(100),
            attack: Some(0),
            defense: Some(0),
            mdefense: Some(0),
            technic: Some(0),
            multi_hp_idx: Some(0),
            multi_hp_num: Some(0),
        };

        FightEntityInfo {
            uid: Some(uid),
            model_id: Some(0),
            skin: Some(0),
            position: Some(0),
            entity_type: Some(3),
            user_id: Some(user_id),
            ex_point: Some(0),
            level: Some(0),
            current_hp: Some(100),
            attr: Some(attr),
            base_attr: Some(attr),
            ex_skill: Some(0),
            shield_value: Some(0),
            expoint_max_add: Some(0),
            buff_harm_statistic: Some(0),
            equip_uid: Some(0),
            ex_skill_level: Some(0),
            ex_skill_point_change: Some(0),
            team_type: Some(team_type),
            enhance_info_box: Some(EnhanceInfoBox {
                uid: Some(uid),
                can_upgrade_ids: vec![],
                upgraded_options: vec![],
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
        }
    }

    fn ex_point_type(hero_id: i32) -> i32 {
        let game = config::configs::get();
        let spec = game
            .character_rank_replace
            .get(hero_id)
            .map(|r| r.unique_skill_point.as_str())
            .or_else(|| {
                game.character
                    .get(hero_id)
                    .map(|c| c.unique_skill_point.as_str())
            });

        spec.and_then(|s| s.split('#').next())
            .and_then(|s| s.trim().parse::<i32>().ok())
            .unwrap_or(0)
    }

    fn career(hero_data: &HeroData) -> i32 {
        config::configs::get()
            .character
            .get(hero_data.record.hero_id)
            .map(|c| c.career)
            .unwrap_or(0)
    }

    fn hero_power_infos(hero_data: &HeroData) -> Vec<PowerInfo> {
        config::configs::get()
            .character
            .get(hero_data.record.hero_id)
            .into_iter()
            .flat_map(|c| parse_power_specs(&c.power_max))
            .filter(|(_, max)| *max > 0)
            .map(|(power_id, max)| PowerInfo {
                power_id: Some(power_id),
                num: Some(0),
                max: Some(max),
            })
            .collect()
    }
}

fn parse_power_specs(spec: &str) -> Vec<(i32, i32)> {
    spec.split('|')
        .filter_map(|entry| {
            let (power_id, max) = entry.trim().split_once('#')?;
            Some((power_id.parse().ok()?, max.parse().ok()?))
        })
        .collect()
}
