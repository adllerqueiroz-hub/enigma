use super::ProfileManager;
use crate::error::AppError;
use database::{db::game::player_infos, models::game::heros::UserHeroModel};
use sonettobuf::AssistHeroInfo;
use sqlx::SqlitePool;

impl ProfileManager {
    pub async fn assist_hero(
        &self,
        db: &SqlitePool,
        hero_id: i32,
        is_friend: bool,
    ) -> Result<Option<(i32, AssistHeroInfo)>, AppError> {
        let hero = match UserHeroModel::new(self.player_id, db.clone())
            .get_hero(hero_id)
            .await
        {
            Ok(hero) => hero,
            Err(_) => return Ok(None),
        };
        let Some(player) = player_infos::get_player_info_data(db, self.player_id).await? else {
            return Ok(None);
        };
        let Some(character) = config::configs::get().character.get(hero.record.hero_id) else {
            return Ok(None);
        };
        let template = hero
            .talent_templates
            .iter()
            .find(|(template, _)| template.template_id == hero.record.use_talent_template_id)
            .or_else(|| hero.talent_templates.first());
        let cubes = template
            .filter(|(_, cubes)| !cubes.is_empty())
            .map(|(_, cubes)| cubes.as_slice())
            .unwrap_or(&hero.talent_cubes);
        let talent_cube_infos = cubes.iter().cloned().map(Into::into).collect();
        let style = template
            .map(|(template, _)| template.style)
            .unwrap_or_default();

        Ok(Some((
            character.career,
            AssistHeroInfo {
                hero_uid: Some(hero.record.uid),
                user_id: Some(self.player_id),
                name: Some(player.user_info.username),
                user_level: Some(player.user_info.level),
                portrait: Some(player.player_info.portrait),
                bg: Some(player.player_info.bg),
                is_friend: Some(is_friend),
                hero_id: Some(hero.record.hero_id),
                level: Some(hero.record.level),
                rank: Some(hero.record.rank),
                skin: Some(hero.record.skin),
                passive_skill_level: hero.passive_skill_levels,
                ex_skill_level: Some(hero.record.ex_skill_level),
                talent: Some(hero.record.talent),
                talent_cube_infos,
                balance_level: Some(hero.record.level),
                is_open_talent: Some(hero.record.talent > 0),
                style: Some(style),
                destiny_rank: Some(hero.record.destiny_rank),
                destiny_level: Some(hero.record.destiny_level),
                destiny_stone: Some(hero.record.destiny_stone),
                extra_str: Some(hero.record.extra_str),
            },
        )))
    }
}
