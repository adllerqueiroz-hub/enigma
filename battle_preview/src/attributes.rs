use std::{collections::HashMap, fs, path::Path};

use battle::engine::entity::stats::{StatInputs, Stats, rank_from_level};
use sonettobuf::{
    Fight, FightEntityInfo, HeroExAttribute, HeroInfo, HeroSpAttribute, HeroUpdatePush,
};

use crate::normalize_live_json;

type PreviewAttributes = (Vec<(i64, HeroExAttribute)>, Vec<(i64, HeroSpAttribute)>);

pub fn preview_attributes(fight: &Fight, battle_path: &Path) -> anyhow::Result<PreviewAttributes> {
    let local = battle_hero_updates(battle_path)?;
    let stats = fight
        .attacker
        .iter()
        .flat_map(|team| team.entitys.iter().chain(team.sub_entitys.iter()))
        .filter_map(|entity| {
            let uid = entity.uid?;
            let hero = local.iter().find(|hero| hero.uid == uid)?;
            let inputs = preview_stat_inputs(entity, hero);
            let stats = Stats::build(&inputs);
            if battle::engine::diagnostics::enabled(battle::engine::diagnostics::TraceArea::Damage) {
                eprintln!(
                    "attribute preview uid={uid} hero={} source=hero-update inputs={inputs:?} stats={stats:?}",
                    entity.model_id.unwrap_or_default(),
                );
            }
            Some((uid, stats))
        })
        .collect::<Vec<_>>();
    Ok((
        stats
            .iter()
            .map(|(uid, stats)| (*uid, stats.ex()))
            .collect(),
        stats
            .iter()
            .map(|(uid, stats)| (*uid, stats.sp()))
            .collect(),
    ))
}

fn battle_hero_updates(path: &Path) -> anyhow::Result<Vec<HeroInfo>> {
    let Some(parent) = path.parent() else {
        return Ok(Vec::new());
    };
    let mut files = fs::read_dir(parent)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("HeroUpdatePush") && name.ends_with(".json"))
        })
        .collect::<Vec<_>>();
    files.sort();

    let mut heroes = HashMap::<i64, HeroInfo>::new();
    for file in files {
        let mut value: serde_json::Value = serde_json::from_str(&fs::read_to_string(file)?)?;
        normalize_live_json(&mut value);
        let update: HeroUpdatePush = serde_json::from_value(value)?;
        for hero in update.hero_updates {
            heroes.insert(hero.uid, hero);
        }
    }
    Ok(heroes.into_values().collect())
}

fn preview_stat_inputs(entity: &FightEntityInfo, hero: &HeroInfo) -> StatInputs {
    let selected_template = hero.use_talent_template_id.unwrap_or_default();
    let template = (selected_template != 0)
        .then(|| {
            hero.talent_templates
                .iter()
                .find(|template| template.id == Some(selected_template))
        })
        .flatten();
    let cubes = template
        .filter(|template| !template.talent_cube_infos.is_empty())
        .map(|template| template.talent_cube_infos.as_slice())
        .unwrap_or(&hero.talent_cube_infos);
    let equip = entity.equips.first();
    StatInputs {
        hero_id: entity.model_id.unwrap_or(hero.hero_id),
        level: entity.level.or(hero.level).unwrap_or_default(),
        rank: hero.rank.unwrap_or_else(|| {
            rank_from_level(
                entity.model_id.unwrap_or(hero.hero_id),
                entity.level.unwrap_or_default(),
            )
        }),
        destiny_rank: entity
            .destiny_rank
            .or(hero.destiny_rank)
            .unwrap_or_default(),
        equip_id: equip.and_then(|equip| equip.equip_id).unwrap_or_default(),
        equip_level: equip.and_then(|equip| equip.equip_lv).unwrap_or_default(),
        equip_break_level: 0,
        talent: hero.talent.unwrap_or(10),
        talent_style: template
            .and_then(|template| template.style)
            .unwrap_or_default(),
        talent_placements: cubes.iter().filter_map(|cube| cube.cube_id).collect(),
    }
}

#[cfg(all(test, feature = "private-fixtures"))]
mod tests {
    use super::*;

    #[test]
    fn battle_local_talent_placements_reconstruct_captured_attributes() {
        crate::init_test_config();
        let battle = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures/battles/battle2/begin_round_1.json");
        let heroes = battle_hero_updates(&battle).unwrap();
        let mut start: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(battle.with_file_name("StartDungeonReply.json")).unwrap(),
        )
        .unwrap();
        normalize_live_json(&mut start);
        let fight: Fight = serde_json::from_value(start["fight"].clone()).unwrap();

        for entity in fight.attacker.iter().flat_map(|team| &team.entitys) {
            let uid = entity.uid.unwrap();
            let Some(hero) = heroes.iter().find(|hero| hero.uid == uid) else {
                continue;
            };
            let stats = Stats::build(&preview_stat_inputs(entity, hero));
            let generated = stats.ex();
            let captured = hero.ex_attr.as_ref().unwrap();
            let generated_sp = stats.sp();
            let captured_sp = hero.sp_attr.as_ref().unwrap();
            let base = entity.attr.as_ref().unwrap();

            assert_eq!(stats.hp, base.hp.unwrap());
            assert_eq!(stats.atk, base.attack.unwrap());
            assert_eq!(stats.def, base.defense.unwrap());
            assert_eq!(stats.mdef, base.mdefense.unwrap());
            assert_eq!(stats.technic, base.technic.unwrap());

            assert_eq!(generated.cri, captured.cri);
            assert_eq!(generated.recri, captured.recri);
            assert_eq!(generated.cri_dmg, captured.cri_dmg);
            assert_eq!(generated.cri_def, captured.cri_def);
            assert_eq!(generated.add_dmg, captured.add_dmg);
            assert_eq!(generated.drop_dmg, captured.drop_dmg);
            // HeroUpdate does not fold destiny-stone attributes into SpAttr.
            if entity.destiny_rank.unwrap_or_default() == 0 {
                assert_eq!(
                    (
                        generated_sp.revive,
                        generated_sp.heal,
                        generated_sp.absorb,
                        generated_sp.defense_ignore,
                        generated_sp.clutch,
                        generated_sp.normal_skill_rate,
                        generated_sp.rebound_dmg,
                        generated_sp.extra_dmg,
                        generated_sp.reuse_dmg,
                    ),
                    (
                        captured_sp.revive,
                        captured_sp.heal,
                        captured_sp.absorb,
                        captured_sp.defense_ignore,
                        captured_sp.clutch,
                        captured_sp.normal_skill_rate,
                        captured_sp.rebound_dmg,
                        captured_sp.extra_dmg,
                        captured_sp.reuse_dmg,
                    ),
                    "special attributes differ for uid={uid}",
                );
            }
        }
    }

    #[test]
    fn tutorial_trial_without_a_hero_update_does_not_invent_extended_attributes() {
        crate::init_test_config();
        let battle = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures/battles/battle62/BeginRoundReply_1.json");
        let mut start: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(battle.with_file_name("StartDungeonReply.json")).unwrap(),
        )
        .unwrap();
        normalize_live_json(&mut start);
        let fight: Fight = serde_json::from_value(start["fight"].clone()).unwrap();

        let (extended, special) = preview_attributes(&fight, &battle).unwrap();

        assert!(extended.is_empty());
        assert!(special.is_empty());
    }
}
