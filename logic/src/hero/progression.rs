use super::*;

impl HeroManager {
    pub async fn level_up(
        self,
        db: &SqlitePool,
        hero_id: i32,
        new_level: i32,
    ) -> Result<(HeroLevelUpReply, HeroInfo), AppError> {
        config::configs::get()
            .character_level(hero_id, new_level)
            .ok_or(AppError::InvalidRequest)?;

        let hero = UserHeroModel::new(self.player_id, db.clone());
        hero.level_up(hero_id, new_level).await?;
        let updated = snapshot(db, hero.get(hero_id).await?).await?;

        Ok((
            HeroLevelUpReply {
                hero_id: Some(hero_id),
                new_level: Some(new_level),
            },
            updated,
        ))
    }

    pub async fn rank_up(
        self,
        db: &SqlitePool,
        hero_id: i32,
    ) -> Result<(HeroRankUpReply, HeroInfo), AppError> {
        let hero = UserHeroModel::new(self.player_id, db.clone());
        let current_rank = hero.get(hero_id).await?.record.rank;
        let new_rank = current_rank + 1;
        if !hero
            .rank_up_with_insight_skin(hero_id, current_rank)
            .await?
        {
            return Err(AppError::InvalidRequest);
        }
        let updated = snapshot(db, hero.get(hero_id).await?).await?;

        Ok((
            HeroRankUpReply {
                hero_id: Some(hero_id),
                new_rank: Some(new_rank),
            },
            updated,
        ))
    }

    pub async fn upgrade_skill(
        self,
        db: &SqlitePool,
        hero_id: i32,
        skill_type: i32,
        levels: i32,
    ) -> Result<(HeroUpgradeSkillReply, HeroInfo, u32), AppError> {
        if skill_type != 3 {
            return Err(AppError::InvalidRequest);
        }

        let hero = UserHeroModel::new(self.player_id, db.clone());
        let current = hero.get(hero_id).await?;
        if current.record.ex_skill_level >= 5 {
            return Err(AppError::InvalidRequest);
        }

        let consume = levels.max(1).min(5 - current.record.ex_skill_level);
        let consumed_item_id = duplicate_item_id(hero_id)?;
        let mut tx = db.begin().await?;
        reward::consume(
            &mut tx,
            self.player_id,
            &reward::RewardSet {
                items: vec![(consumed_item_id, consume)],
                ..Default::default()
            },
        )
        .await?;
        if !hero
            .upgrade_ex_skill_in_transaction(
                &mut tx,
                hero_id,
                current.record.ex_skill_level,
                consume,
            )
            .await?
        {
            return Err(AppError::InvalidRequest);
        }
        tx.commit().await?;
        let updated = snapshot(db, hero.get(hero_id).await?).await?;

        Ok((HeroUpgradeSkillReply {}, updated, consumed_item_id))
    }
}

pub(super) fn duplicate_item_id(hero_id: i32) -> Result<u32, AppError> {
    let character = config::configs::get()
        .character
        .get(hero_id)
        .ok_or(AppError::InvalidRequest)?;

    reward::parse(&character.duplicate_item)
        .items
        .first()
        .map(|(id, _)| *id)
        .ok_or(AppError::InvalidRequest)
}
