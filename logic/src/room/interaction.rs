use super::*;

impl RoomManager {
    pub async fn character_interaction_info(
        &self,
        db: &SqlitePool,
    ) -> Result<GetCharacterInteractionInfoReply, AppError> {
        Ok(GetCharacterInteractionInfoReply {
            infos: character_interactions::get_character_interactions(db, self.player_id)
                .await?
                .into_iter()
                .map(Into::into)
                .collect(),
            interaction_count: Some(
                character_interactions::get_interaction_count(db, self.player_id).await?,
            ),
        })
    }

    pub async fn start_character_interaction(
        &self,
        db: &SqlitePool,
        tables: &config::GameDB,
        interaction_id: i32,
    ) -> Result<StartCharacterInteractionReply, AppError> {
        validate_character_interaction(db, tables, self.player_id, interaction_id).await?;
        if !character_interactions::start_interaction(db, self.player_id, interaction_id).await? {
            return Err(AppError::InvalidRequest);
        }
        Ok(StartCharacterInteractionReply {
            id: Some(interaction_id),
        })
    }

    pub async fn complete_character_interaction(
        &self,
        db: &SqlitePool,
        tables: &config::GameDB,
        interaction_id: i32,
        select_ids: Vec<i32>,
    ) -> Result<RoomReward<GetCharacterInteractionBonusReply>, AppError> {
        let interaction =
            validate_character_interaction(db, tables, self.player_id, interaction_id).await?;
        let mut unique = BTreeSet::new();
        if select_ids.iter().any(|id| {
            !unique.insert(*id)
                || tables
                    .room_character_dialog_select
                    .get(*id)
                    .is_none_or(|select| select.dialog_id != interaction.dialog_id)
        }) {
            return Err(AppError::InvalidRequest);
        }
        let reward_set = reward::parse(&interaction.reward);
        let material_changes = reward_set.material_changes();
        let mut tx = db.begin().await?;
        if !character_interactions::complete_interaction_in_transaction(
            &mut tx,
            self.player_id,
            interaction_id,
            &select_ids,
        )
        .await?
        {
            return Err(AppError::InvalidRequest);
        }
        let rewards = reward::apply_in_transaction(&mut tx, db, self.player_id, reward_set).await?;
        tx.commit().await?;
        Ok(RoomReward {
            reply: GetCharacterInteractionBonusReply {
                id: Some(interaction_id),
                select_ids,
            },
            rewards,
            material_changes,
        })
    }
}

async fn validate_character_interaction<'a>(
    db: &SqlitePool,
    tables: &'a config::GameDB,
    player_id: i64,
    interaction_id: i32,
) -> Result<&'a config::room_character_interaction::RoomCharacterInteraction, AppError> {
    let interaction = tables
        .room_character_interaction
        .get(interaction_id)
        .ok_or(AppError::InvalidRequest)?;
    if interaction.weather != 0
        || !room_interaction_condition_matches(db, player_id, &interaction.condition_str).await?
    {
        return Err(AppError::InvalidRequest);
    }
    let heroes: BTreeSet<_> = room_ob::get_heroes(db, player_id, &[])
        .await?
        .into_iter()
        .map(|hero| hero.hero_id)
        .collect();
    if !heroes.contains(&interaction.hero_id)
        || (interaction.relate_hero_id != 0 && !heroes.contains(&interaction.relate_hero_id))
    {
        return Err(AppError::InvalidRequest);
    }
    if interaction.building_id != 0
        && !buildings::get_placed_buildings(db, player_id)
            .await?
            .iter()
            .any(|building| building.define_id == interaction.building_id)
    {
        return Err(AppError::InvalidRequest);
    }
    Ok(interaction)
}

async fn room_interaction_condition_matches(
    db: &SqlitePool,
    player_id: i64,
    condition: &str,
) -> Result<bool, AppError> {
    if condition.is_empty() {
        return Ok(true);
    }
    let heroes = UserHeroModel::new(player_id, db.clone());
    for alternatives in condition.split(" or ") {
        let mut matches = true;
        for clause in alternatives.split(" and ") {
            let Some((name, value)) = clause.trim().split_once('=') else {
                return Ok(false);
            };
            let clause_matches = match name.trim() {
                "EpisodeFinish" => {
                    let Ok(episode_id) = value.trim().parse() else {
                        return Ok(false);
                    };
                    dungeons::episode_star(db, player_id, episode_id).await? > 0
                }
                "HeroSkinId" => {
                    let Some((hero_id, skin_id)) = value.trim().split_once('#') else {
                        return Ok(false);
                    };
                    let (Ok(hero_id), Ok(skin_id)) = (hero_id.parse(), skin_id.parse()) else {
                        return Ok(false);
                    };
                    heroes.equipped_skin(hero_id).await? == Some(skin_id)
                }
                _ => false,
            };
            if !clause_matches {
                matches = false;
                break;
            }
        }
        if matches {
            return Ok(true);
        }
    }
    Ok(false)
}
