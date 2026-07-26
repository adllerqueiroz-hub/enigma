use super::*;

pub async fn refresh_assist(
    db: &SqlitePool,
    player_id: i64,
    request: RefreshAssistRequest,
) -> Result<RefreshAssistReply, AppError> {
    let Some(hero_id) = request.ext.as_deref().and_then(|value| value.parse().ok()) else {
        tracing::warn!(
            assist_type = request.assist_type,
            ext = request.ext,
            "assist roster selector is not implemented"
        );
        return Ok(RefreshAssistReply {
            assist_type: request.assist_type,
            assist_hero_careers: vec![],
            ext: request.ext,
        });
    };

    let friends = database::db::game::friends::get_friend_ids(db, player_id).await?;
    let friend_ids = friends.iter().map(|id| *id as i64).collect::<HashSet<_>>();
    let mut candidate_ids = friends;
    candidate_ids
        .extend(database::db::game::friends::get_recommended_ids(db, player_id, 20).await?);

    let mut careers = BTreeMap::<i32, Vec<AssistHeroInfo>>::new();
    let mut seen = HashSet::new();
    for candidate_id in candidate_ids
        .into_iter()
        .map(|id| id as i64)
        .filter(|id| seen.insert(*id))
    {
        let Ok(hero) = database::models::game::heros::UserHeroModel::new(candidate_id, db.clone())
            .get_hero(hero_id)
            .await
        else {
            continue;
        };
        let Some(player) =
            database::db::game::player_infos::get_player_info_data(db, candidate_id).await?
        else {
            continue;
        };
        let Some(character) = configs::get().character.get(hero.record.hero_id) else {
            continue;
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

        careers
            .entry(character.career)
            .or_default()
            .push(AssistHeroInfo {
                hero_uid: Some(hero.record.uid),
                user_id: Some(candidate_id),
                name: Some(player.user_info.username),
                user_level: Some(player.user_info.level),
                portrait: Some(player.player_info.portrait),
                bg: Some(player.player_info.bg),
                is_friend: Some(friend_ids.contains(&candidate_id)),
                hero_id: Some(hero.record.hero_id),
                level: Some(hero.record.level),
                rank: Some(hero.record.rank),
                skin: Some(hero.record.skin),
                passive_skill_level: hero.passive_skill_levels.clone(),
                ex_skill_level: Some(hero.record.ex_skill_level),
                talent: Some(hero.record.talent),
                talent_cube_infos: cubes.iter().cloned().map(Into::into).collect(),
                balance_level: Some(hero.record.level),
                is_open_talent: Some(hero.record.talent > 0),
                style: Some(
                    template
                        .map(|(template, _)| template.style)
                        .unwrap_or_default(),
                ),
                destiny_rank: Some(hero.record.destiny_rank),
                destiny_level: Some(hero.record.destiny_level),
                destiny_stone: Some(hero.record.destiny_stone),
                extra_str: Some(hero.record.extra_str.clone()),
            });
    }

    Ok(RefreshAssistReply {
        assist_type: request.assist_type,
        assist_hero_careers: careers
            .into_iter()
            .map(|(career, assist_hero_infos)| AssistHeroCareerNo {
                career: Some(career),
                assist_hero_infos,
            })
            .collect(),
        ext: request.ext,
    })
}
