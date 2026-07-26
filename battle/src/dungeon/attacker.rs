use crate::engine::{
    entity::{
        builder::EntityBuilder,
        stats::{StatInputs, Stats},
    },
    fight::{defender::Defender, team::Team},
    manager::eureka::PowerType,
};
use anyhow::{Result, ensure};
use database::models::game::{
    equipment::{Equipment, UserEquipmentModel},
    heros::{HeroData, HeroModel, UserHeroModel, get_hero_by_uid},
};
use serde::Deserialize;
use sonettobuf::{
    AssistBossInfo, AssistBossSkillInfo, EnhanceInfoBox, EquipRecord, FightEntityInfo, FightTeam,
    HeroExAttribute, HeroSpAttribute, PowerInfo,
};
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};

const SUPPORT_UID: i64 = -1;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ComposeParams {
    theme_id: i32,
    layer_id: i32,
    plane_id: i32,
    support_id: i32,
    #[serde(default)]
    support_assist_uid: i64,
    #[serde(default)]
    support_assist_type: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ComposeGroupParams {
    buff_params_str: String,
    #[serde(default)]
    assist_data_map: Vec<Option<HashMap<String, ComposeAssist>>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ComposeAssist {
    hero_uid: i64,
    hero_id: i32,
    level: i32,
    skin: i32,
    ex_skill_level: i32,
    assist_type: i32,
}

pub struct Attacker;

pub struct BuiltAttacker {
    pub team: FightTeam,
    pub ex_attributes: Vec<(i64, HeroExAttribute)>,
    pub sp_attributes: Vec<(i64, HeroSpAttribute)>,
    pub reserved_uid_offset: usize,
}

impl Attacker {
    pub async fn get(
        pool: &SqlitePool,
        user_id: i64,
        episode_id: i32,
        battle_id: i32,
        fight_group: &sonettobuf::FightGroup,
        params: Option<&str>,
    ) -> Result<BuiltAttacker> {
        let battle = config::configs::get()
            .battle
            .get(battle_id)
            .ok_or_else(|| anyhow::anyhow!("unknown battle {battle_id}"))?;
        let aid_ids = configured_aid_ids(&battle.aid)?;
        validate_composition(
            fight_group,
            battle.role_num,
            battle.player_max,
            aid_ids.len(),
        )?;
        let use_configured_aids = !aid_ids.is_empty()
            && fight_group
                .hero_list
                .iter()
                .chain(&fight_group.sub_hero_list)
                .all(|uid| *uid == 0);

        let mut entitys = Vec::new();
        let mut sub_entitys = Vec::new();
        let mut ex_attributes = Vec::new();
        let mut sp_attributes = Vec::new();
        let hero = UserHeroModel::new(user_id, pool.clone());

        if use_configured_aids {
            for (index, monster_id) in aid_ids.iter().copied().enumerate() {
                let uid = -i64::try_from(index + 1)?;
                entitys.push(Defender::build_monster_with_uid(
                    monster_id,
                    uid,
                    (index + 1) as i32,
                    1,
                )?);
            }
        }

        for (position, hero_uid) in fight_group.hero_list.iter().enumerate() {
            if *hero_uid == 0 {
                continue;
            }
            if let Some(entity) = configured_aid(&aid_ids, *hero_uid, (position + 1) as i32)? {
                entitys.push(entity);
                continue;
            }

            let hero_data = hero.get_uid(*hero_uid).await?;
            let equip = Self::fetch_equip(pool, &hero_data, fight_group).await;
            let stats = crate::engine::entity::stats::Stats::build(
                &crate::engine::entity::stats::StatInputs::from_hero_data(
                    &hero_data,
                    equip.as_ref(),
                ),
            );
            ex_attributes.push((hero_data.record.uid, stats.ex()));
            sp_attributes.push((hero_data.record.uid, stats.sp()));

            let mut builder = EntityBuilder::new(hero_data, (position + 1) as i32, 1, false);
            if let Some(equip) = equip {
                builder = builder.with_equip(equip);
            }
            entitys.push(builder.build());
        }

        for hero_uid in fight_group.sub_hero_list.iter() {
            if *hero_uid == 0 {
                continue;
            }
            if let Some(entity) = configured_aid(&aid_ids, *hero_uid, -1)? {
                sub_entitys.push(entity);
                continue;
            }

            let hero_data = hero.get_uid(*hero_uid).await?;
            let equip = Self::fetch_equip(pool, &hero_data, fight_group).await;
            let stats = crate::engine::entity::stats::Stats::build(
                &crate::engine::entity::stats::StatInputs::from_hero_data(
                    &hero_data,
                    equip.as_ref(),
                ),
            );
            ex_attributes.push((hero_data.record.uid, stats.ex()));
            sp_attributes.push((hero_data.record.uid, stats.sp()));

            let mut builder = EntityBuilder::new(hero_data, -1, 1, true);
            if let Some(equip) = equip {
                builder = builder.with_equip(equip);
            }
            sub_entitys.push(builder.build());
        }

        let player_entity = EntityBuilder::player(user_id, 1);
        let skill_infos = Team::get_player_skills(fight_group.cloth_id);

        let mut team = Team::build(
            entitys,
            sub_entitys,
            player_entity,
            Some(15),
            fight_group.cloth_id,
            skill_infos,
        );
        if let Some((support, info, ex, sp)) =
            Self::compose_support(pool, user_id, episode_id, fight_group, params).await?
        {
            team.assist_boss = Some(support);
            team.assist_boss_info = Some(info);
            ex_attributes.push((SUPPORT_UID, ex));
            sp_attributes.push((SUPPORT_UID, sp));
        }
        let reserved_uid_offset = reserved_uid_offset(
            fight_group,
            if use_configured_aids {
                aid_ids.len()
            } else {
                0
            },
            team.assist_boss.is_some() || fight_group.assist_boss_id.unwrap_or_default() != 0,
        )?;

        Ok(BuiltAttacker {
            team,
            ex_attributes,
            sp_attributes,
            reserved_uid_offset,
        })
    }

    async fn fetch_equip(
        pool: &SqlitePool,
        hero_data: &HeroData,
        fight_group: &sonettobuf::FightGroup,
    ) -> Option<Equipment> {
        let equip_uid = fight_group
            .equips
            .iter()
            .find(|equip| equip.hero_uid == Some(hero_data.record.uid))
            .and_then(|equip| equip.equip_uid.first().copied())
            .filter(|uid| *uid != 0)
            .unwrap_or(hero_data.record.default_equip_uid);

        UserEquipmentModel::new(hero_data.record.user_id, pool.clone())
            .get_equip(equip_uid)
            .await
            .ok()
    }

    async fn compose_support(
        pool: &SqlitePool,
        user_id: i64,
        episode_id: i32,
        fight_group: &sonettobuf::FightGroup,
        params: Option<&str>,
    ) -> Result<
        Option<(
            FightEntityInfo,
            AssistBossInfo,
            HeroExAttribute,
            HeroSpAttribute,
        )>,
    > {
        let tables = config::configs::get();
        if !tables
            .tower_compose_episode
            .iter()
            .any(|row| row.episode_id == episode_id)
        {
            return Ok(None);
        }
        let params: ComposeParams = serde_json::from_str(
            params.ok_or_else(|| anyhow::anyhow!("missing Tower Compose battle params"))?,
        )?;
        ensure!(tables.tower_compose_episode.iter().any(|row| {
            row.episode_id == episode_id
                && row.theme_id == params.theme_id
                && row.layer_id == params.layer_id
                && row.plane == params.plane_id
        }));
        if params.support_id == 0 {
            return Ok(None);
        }

        let group: ComposeGroupParams = serde_json::from_str(
            fight_group
                .params
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("missing Compose group params"))?,
        )?;
        ensure!(
            selected_support(&group.buff_params_str, params.plane_id) == Some(params.support_id)
        );
        let support = tables
            .tower_compose_support
            .get(params.support_id)
            .ok_or_else(|| {
                anyhow::anyhow!("unknown Tower Compose support {}", params.support_id)
            })?;
        ensure!(support.theme_id == params.theme_id);

        let hero = if params.support_assist_uid == 0 {
            UserHeroModel::new(user_id, pool.clone())
                .get_hero(support.hero_id)
                .await?
        } else {
            let assist = group
                .assist_data_map
                .get(params.theme_id.saturating_sub(1) as usize)
                .and_then(Option::as_ref)
                .and_then(|planes| planes.get(&params.plane_id.to_string()))
                .ok_or_else(|| anyhow::anyhow!("missing selected Tower Compose assist"))?;
            ensure!(assist.hero_uid == params.support_assist_uid);
            ensure!(assist.hero_id == support.hero_id);
            ensure!(assist.assist_type == params.support_assist_type);
            let hero = get_hero_by_uid(pool, assist.hero_uid).await?;
            ensure!(hero.record.hero_id == assist.hero_id);
            ensure!(hero.record.level == assist.level);
            ensure!(hero.record.skin == assist.skin);
            ensure!(hero.record.ex_skill_level == assist.ex_skill_level);
            hero
        };
        ensure!(hero.record.hero_id == support.hero_id);
        ensure!(hero.record.ex_skill_level == support.lv);

        let stats = Stats::build(&StatInputs::from_hero_data(&hero, None));
        let attr = stats.base();
        let entity = FightEntityInfo {
            uid: Some(SUPPORT_UID),
            model_id: Some(support.id),
            skin: Some(hero.record.skin),
            position: Some(0),
            entity_type: Some(5),
            user_id: Some(user_id),
            ex_point: Some(0),
            level: Some(hero.record.level),
            current_hp: attr.hp,
            attr: Some(attr),
            passive_skill: ids(&support.passive_skills),
            ex_skill: Some(0),
            shield_value: Some(0),
            expoint_max_add: Some(0),
            buff_harm_statistic: Some(0),
            equip_uid: Some(0),
            trial_equip: Some(EquipRecord::default()),
            ex_skill_level: Some(hero.record.ex_skill_level),
            power_infos: vec![PowerInfo {
                power_id: Some(PowerType::AssistBoss.id()),
                num: Some(support.res_init_val),
                max: Some(support.res_max_val),
            }],
            base_attr: Some(attr),
            ex_skill_point_change: Some(0),
            team_type: Some(1),
            enhance_info_box: Some(EnhanceInfoBox {
                uid: Some(SUPPORT_UID),
                ..Default::default()
            }),
            trial_id: Some(0),
            career: tables
                .character
                .get(support.hero_id)
                .map(|hero| hero.career),
            status: Some(0),
            guard: Some(-1),
            sub_cd: Some(0),
            ex_point_type: Some(0),
            destiny_stone: Some(0),
            destiny_rank: Some(0),
            custom_unit_id: Some(support.active_type),
            ..Default::default()
        };
        let info = AssistBossInfo {
            skills: active_skills(&support.active_skills),
            curr_cd: Some(0),
            cd_cfg: Some(support.cold_time),
            form_id: Some(0),
            round_use_limit: Some(1),
            exceed_use_free: Some(0),
            params: None,
            r#type: Some(support.active_type),
        };
        Ok(Some((entity, info, stats.ex(), stats.sp())))
    }
}

fn validate_composition(
    fight_group: &sonettobuf::FightGroup,
    role_num: i32,
    player_max: i32,
    aid_count: usize,
) -> Result<()> {
    let role_num = role_num.max(0) as usize;
    let player_max = player_max.max(0) as usize;
    ensure!(fight_group.hero_list.len() <= player_max);
    ensure!(fight_group.sub_hero_list.len() <= role_num.saturating_sub(player_max));

    let selected = fight_group
        .hero_list
        .iter()
        .chain(&fight_group.sub_hero_list)
        .copied()
        .filter(|uid| *uid != 0)
        .collect::<Vec<_>>();
    ensure!(selected.len() <= role_num);
    if selected.is_empty() {
        ensure!(aid_count <= role_num);
        ensure!(aid_count <= player_max);
    }
    ensure!(
        selected
            .iter()
            .all(|uid| { *uid > 0 || (*uid < 0 && uid.unsigned_abs() <= aid_count as u64) })
    );
    ensure!(selected.iter().copied().collect::<HashSet<_>>().len() == selected.len());
    Ok(())
}

fn configured_aid(aid_ids: &[i32], uid: i64, position: i32) -> Result<Option<FightEntityInfo>> {
    if uid >= 0 {
        return Ok(None);
    }
    let index = usize::try_from(uid.unsigned_abs() - 1)
        .map_err(|_| anyhow::anyhow!("configured aid uid {uid} is out of range"))?;
    let monster_id = *aid_ids
        .get(index)
        .ok_or_else(|| anyhow::anyhow!("configured aid uid {uid} is not declared by the battle"))?;
    Ok(Some(Defender::build_monster_with_uid(
        monster_id, uid, position, 1,
    )?))
}

fn configured_aid_ids(raw: &str) -> Result<Vec<i32>> {
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    raw.split('#')
        .map(|value| {
            value
                .parse()
                .map_err(|_| anyhow::anyhow!("invalid configured aid monster id '{value}'"))
        })
        .collect()
}

fn reserved_uid_offset(
    fight_group: &sonettobuf::FightGroup,
    configured_aid_count: usize,
    has_fixed_support: bool,
) -> Result<usize> {
    let aid_offset = fight_group
        .hero_list
        .iter()
        .chain(&fight_group.sub_hero_list)
        .filter(|uid| **uid < 0)
        .filter_map(|uid| usize::try_from(uid.unsigned_abs()).ok())
        .max()
        .unwrap_or_default()
        .max(configured_aid_count);
    ensure!(
        !has_fixed_support || aid_offset == 0,
        "configured aid uid collides with fixed support uid"
    );
    Ok(aid_offset.max(usize::from(has_fixed_support)))
}

fn selected_support(raw: &str, plane_id: i32) -> Option<i32> {
    let plane = if plane_id == 0 {
        raw
    } else {
        raw.split('|').nth(plane_id.saturating_sub(1) as usize)?
    };
    plane.split('#').next()?.parse().ok()
}

fn ids(raw: &str) -> Vec<i32> {
    raw.split(['#', '|'])
        .filter_map(|value| value.parse().ok())
        .collect()
}

fn active_skills(raw: &str) -> Vec<AssistBossSkillInfo> {
    raw.split('|')
        .filter_map(|entry| {
            let mut values = entry.split('#').filter_map(|value| value.parse().ok());
            Some(AssistBossSkillInfo {
                skill_id: Some(values.next()?),
                need_power: Some(values.next()?),
                power_low: Some(values.next()?),
                power_high: Some(values.next()?),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{configured_aid_ids, reserved_uid_offset, selected_support, validate_composition};
    use database::models::game::heros::{HeroModel, UserHeroModel};

    #[test]
    fn selects_support_from_the_active_compose_plane() {
        assert_eq!(selected_support("30#40#50", 0), Some(30));
        assert_eq!(selected_support("31#41#51|32#42#52", 2), Some(32));
    }

    #[test]
    fn validates_configured_team_slots_and_unique_hero_uids() {
        let valid = sonettobuf::FightGroup {
            hero_list: vec![10, 11, 12],
            sub_hero_list: vec![13],
            ..Default::default()
        };
        assert!(validate_composition(&valid, 4, 3, 0).is_ok());

        let duplicate = sonettobuf::FightGroup {
            sub_hero_list: vec![12],
            ..valid.clone()
        };
        assert!(validate_composition(&duplicate, 4, 3, 0).is_err());

        let oversized = sonettobuf::FightGroup {
            hero_list: vec![10, 11, 12, 13],
            ..Default::default()
        };
        assert!(validate_composition(&oversized, 4, 3, 0).is_err());

        let configured_aids = sonettobuf::FightGroup {
            hero_list: vec![-1, -2],
            ..Default::default()
        };
        assert!(validate_composition(&configured_aids, 3, 3, 2).is_ok());
        assert!(validate_composition(&configured_aids, 3, 3, 1).is_err());
        assert_eq!(
            configured_aid_ids("100102#100101").unwrap(),
            vec![100102, 100101]
        );
        assert!(configured_aid_ids("100102#bad#100101").is_err());
        assert!(reserved_uid_offset(&configured_aids, 0, true).is_err());
        assert_eq!(
            reserved_uid_offset(&sonettobuf::FightGroup::default(), 0, true).unwrap(),
            1
        );
        assert!(reserved_uid_offset(&sonettobuf::FightGroup::default(), 2, true).is_err());
    }

    #[tokio::test]
    async fn configured_aids_build_as_allied_monsters_and_reserve_their_uids() {
        crate::test_support::init_config();
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
        let fight_group = sonettobuf::FightGroup::default();

        let built = super::super::build_fight(&pool, 1, 10002, 1002, false, &fight_group, None)
            .await
            .unwrap();
        let mut runtime = crate::engine::runtime::BattleRuntime::new(built.fight.clone());
        let round = runtime.start_round().unwrap();
        let cards = runtime.card_info_push();
        let deal = round
            .team_a_cards1
            .iter()
            .map(|card| (card.uid.unwrap(), card.skill_id.unwrap()))
            .collect::<Vec<_>>();
        assert_eq!(
            deal,
            vec![
                (-1, 30250121),
                (-1, 30250121),
                (-1, 30250121),
                (-2, 30230111),
                (-2, 30230111),
                (-2, 30230121),
                (-1, 30250121),
            ]
        );
        assert_eq!(
            cards
                .deal_card_group
                .iter()
                .map(|card| (card.uid.unwrap(), card.skill_id.unwrap()))
                .collect::<Vec<_>>(),
            deal
        );
        let opening = cards
            .card_group
            .into_iter()
            .map(|card| (card.uid.unwrap(), card.skill_id.unwrap()))
            .collect::<Vec<_>>();
        assert_eq!(
            opening,
            vec![
                (-1, 30250122),
                (-1, 30250121),
                (-2, 30230112),
                (-2, 30230121),
                (-1, 30250121),
            ]
        );
        assert_eq!(opening[4], opening[1]);
        let attacker = built.fight.attacker.unwrap();
        let defender = built.fight.defender.unwrap();

        assert_eq!(
            attacker
                .entitys
                .iter()
                .map(|entity| (entity.uid, entity.model_id, entity.team_type))
                .collect::<Vec<_>>(),
            vec![
                (Some(-1), Some(100102), Some(1)),
                (Some(-2), Some(100101), Some(1)),
            ]
        );
        assert_eq!(
            defender
                .entitys
                .iter()
                .map(|entity| (entity.uid, entity.model_id))
                .collect::<Vec<_>>(),
            vec![(Some(-3), Some(100105)), (Some(-4), Some(100106))]
        );

        let built = super::super::build_fight(&pool, 1, 10003, 1003, false, &fight_group, None)
            .await
            .unwrap();
        assert_eq!(
            built
                .fight
                .attacker
                .unwrap()
                .entitys
                .iter()
                .map(|entity| (entity.uid, entity.model_id))
                .collect::<Vec<_>>(),
            vec![(Some(-1), Some(100109))]
        );
        assert_eq!(built.fight.defender.unwrap().entitys[0].uid, Some(-2));
    }

    #[tokio::test]
    async fn compose_support_is_built_before_defender_uids() {
        crate::test_support::init_config();
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
        database::run_migrations(&pool).await.unwrap();
        for (id, name) in [(1_i64, "owner"), (2, "helper")] {
            sqlx::query(
                "INSERT INTO users (id, username, created_at, updated_at) VALUES (?, ?, 0, 0)",
            )
            .bind(id)
            .bind(name)
            .execute(&pool)
            .await
            .unwrap();
            database::db::starter_data::load_all_starter_data(&pool, id)
                .await
                .unwrap();
        }
        let hero_uid = UserHeroModel::new(2, pool.clone())
            .create_hero(3066)
            .await
            .unwrap();
        let hero = UserHeroModel::new(2, pool.clone())
            .get_uid(hero_uid)
            .await
            .unwrap();
        let group = sonettobuf::FightGroup {
            params: Some(format!(
                r#"{{"heroList":["0","0","0","0"],"assistDataMap":[{{"0":{{"level":{},"skin":{},"exSkillLevel":{},"heroUid":{},"heroId":3066,"assistType":6}}}}],"buffParamsStr":"306600#0#0"}}"#,
                hero.record.level, hero.record.skin, hero.record.ex_skill_level, hero_uid
            )),
            ..Default::default()
        };
        let params = format!(
            r#"{{"supportAssistUid":{},"layerId":1,"supportId":306600,"supportAssistType":6,"themeId":1,"planeId":0}}"#,
            hero_uid
        );

        let built =
            super::super::build_fight(&pool, 1, 90400101, 9001101, false, &group, Some(&params))
                .await
                .unwrap();
        let attacker = built.fight.attacker.unwrap();
        assert_eq!(attacker.assist_boss.unwrap().model_id, Some(306600));
        assert_eq!(built.fight.defender.unwrap().entitys[0].uid, Some(-2),);
    }
}
