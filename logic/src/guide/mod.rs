use crate::{error::AppError, reward, types::hero_group_snapshot_type::HeroGroupSnapshotType};
use database::{
    db::game::{dungeons, guides, hero_group_snapshots, hero_groups, open_infos, stories},
    models::game::heros::UserHeroModel,
};
use sonettobuf::{FinishGuideReply, GetGuideInfoReply, GuideInfo, UpdateHeroGroupSnapshotPush};
use sqlx::SqlitePool;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum GuideActionKind {
    PlayStory = 101,
    EnterEpisode = 102,
    OpenView = 105,
    WaitForStory = 204,
}

impl GuideActionKind {
    fn tag(self) -> &'static str {
        match self {
            Self::PlayStory => "101",
            Self::EnterEpisode => "102",
            Self::OpenView => "105",
            Self::WaitForStory => "204",
        }
    }
}

pub struct GuideCompletion {
    pub reply: FinishGuideReply,
    pub guide_info: GuideInfo,
    pub rewards: reward::AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
    pub group_snapshot: Option<UpdateHeroGroupSnapshotPush>,
}

fn story_requirement(action: &str) -> Option<i32> {
    action.split('|').find_map(|action| {
        let mut fields = action.split('#');
        (fields.next() == Some(GuideActionKind::WaitForStory.tag()))
            .then(|| fields.next()?.parse().ok())
            .flatten()
    })
}

fn action_ids(action: &str, kind: GuideActionKind) -> impl Iterator<Item = i32> + '_ {
    action.split('|').filter_map(move |action| {
        let mut fields = action.split('#');
        (fields.next() == Some(kind.tag()))
            .then(|| fields.next()?.parse().ok())
            .flatten()
    })
}

fn hero_reward_after_step(guide_id: i32, step_id: i32) -> Option<i32> {
    let steps = &config::configs::get().guide_step;
    let reward_step = steps
        .iter()
        .filter(|step| step.id == guide_id && step.step_id > step_id)
        .min_by_key(|step| step.step_id)?;

    reward_step.action.split('|').find_map(|action| {
        let mut fields = action.split('#');
        (fields.next() == Some(GuideActionKind::OpenView.tag())
            && fields.next() == Some("CharacterGetView"))
        .then(|| fields.next()?.parse().ok())
        .flatten()
    })
}

fn stored_step_after(guide_id: i32, step_id: i32) -> i32 {
    if config::configs::get()
        .guide_step
        .iter()
        .any(|step| step.id == guide_id && step.step_id > step_id && step.key_step != 0)
    {
        step_id
    } else {
        -1
    }
}

fn teaching_rewards(guide_id: i32, step_id: i32) -> reward::RewardSet {
    config::configs::get()
        .teaching_summon
        .iter()
        .filter(|row| row.grant_guide_id == guide_id && row.grant_step_id == step_id)
        .map(|row| reward::parse(&row.grant_reward))
        .fold(reward::RewardSet::default(), |mut rewards, reward| {
            rewards.extend(reward);
            rewards
        })
}

fn episode_condition(value: &str) -> Option<i32> {
    value.strip_prefix("EpisodeFinish#")?.parse().ok()
}

fn configured_clear_star(episode: &config::episode::Episode) -> i32 {
    let advanced = config::configs::get()
        .battle
        .get(episode.battle_id)
        .map(|battle| {
            battle
                .advanced_condition
                .split('|')
                .filter(|condition| !condition.is_empty())
                .count() as i32
        })
        .unwrap_or_default();
    1 + advanced
}

async fn complete_skipped_episode(
    db: &SqlitePool,
    player_id: i64,
    episode_id: i32,
) -> Result<(), AppError> {
    let episode = config::configs::get()
        .episode
        .get(episode_id)
        .ok_or(AppError::InvalidRequest)?;
    let star = configured_clear_star(episode);
    let mut tx = db.begin().await?;
    let previous_star =
        dungeons::episode_star_in_transaction(&mut tx, player_id, episode_id).await?;
    if previous_star < star {
        dungeons::update_dungeon_progress_in_transaction(
            &mut tx,
            player_id,
            episode.chapter_id,
            episode.id,
            star,
        )
        .await?;
        let first_pass = previous_star == 0;
        let completion = crate::dungeon::completion_rewards(
            episode,
            first_pass,
            previous_star,
            star,
            i32::from(first_pass),
        );
        reward::RewardManager::new(player_id)
            .apply_dungeon_in_transaction(&mut tx, completion.rewards)
            .await?;
        open_infos::reconcile_progression_in_transaction(&mut tx, player_id).await?;
    }
    for story_id in [episode.before_story, episode.after_story]
        .into_iter()
        .filter(|id| *id > 0)
    {
        stories::finish_story_in_transaction(&mut tx, player_id, story_id).await?;
    }
    tx.commit().await?;
    Ok(())
}

async fn apply_guide_world_progress(
    db: &SqlitePool,
    player_id: i64,
    guide_id: i32,
) -> Result<i32, AppError> {
    let tables = config::configs::get();
    let steps = tables
        .guide_step
        .iter()
        .filter(|step| step.id == guide_id)
        .collect::<Vec<_>>();
    for story_id in steps.iter().flat_map(|step| {
        action_ids(&step.action, GuideActionKind::PlayStory)
            .chain(action_ids(&step.action, GuideActionKind::WaitForStory))
    }) {
        stories::finish_story(db, player_id, story_id).await?;
    }
    for episode_id in steps
        .iter()
        .flat_map(|step| action_ids(&step.action, GuideActionKind::EnterEpisode))
    {
        complete_skipped_episode(db, player_id, episode_id).await?;
    }

    steps
        .into_iter()
        .filter(|step| step.key_step != 0)
        .max_by_key(|step| step.step_id)
        .map(|step| step.step_id)
        .ok_or(AppError::InvalidRequest)
}

#[derive(Clone, Copy, Debug)]
pub struct GuideManager {
    player_id: i64,
}

impl GuideManager {
    pub fn new(player_id: i64) -> Self {
        Self { player_id }
    }

    pub async fn get_info(&self, db: &SqlitePool) -> Result<GetGuideInfoReply, AppError> {
        let guide_infos = guides::get_all_guide_progress(db, self.player_id).await?;

        Ok(GetGuideInfoReply {
            guide_infos: guide_infos.into_iter().map(Into::into).collect(),
        })
    }

    pub async fn skip_initial_tutorial(&self, db: &SqlitePool) -> Result<(), AppError> {
        let tables = config::configs::get();
        let guide = tables
            .guide
            .iter()
            .find(|guide| guide.is_online != 0 && guide.trigger == "PlayerLv#1")
            .ok_or(AppError::InvalidRequest)?;
        let initial_complete = guides::get_guide_progress(db, self.player_id, guide.id)
            .await?
            .is_some_and(|progress| progress.step_id == -1);
        let completion_step = apply_guide_world_progress(db, self.player_id, guide.id).await?;
        if !initial_complete {
            self.finish(db, guide.id, completion_step).await?;
        }

        let tutorial_episodes = tables
            .guide_step
            .iter()
            .filter(|step| step.id == guide.id)
            .flat_map(|step| action_ids(&step.action, GuideActionKind::EnterEpisode))
            .collect::<Vec<_>>();
        let teaching_guides = tables
            .teaching_summon
            .iter()
            .filter(|teaching| teaching.grant_guide_id == guide.id)
            .map(|teaching| teaching.id)
            .collect::<Vec<_>>();
        for prerequisite in tables.guide.iter().filter(|candidate| {
            !teaching_guides.contains(&candidate.id)
                && episode_condition(&candidate.trigger)
                    .is_some_and(|episode| tutorial_episodes.contains(&episode))
                && episode_condition(&candidate.invalid).is_some()
        }) {
            let progress = guides::get_guide_progress(db, self.player_id, prerequisite.id).await?;
            complete_skipped_episode(
                db,
                self.player_id,
                episode_condition(&prerequisite.invalid).unwrap(),
            )
            .await?;
            let completion_step =
                apply_guide_world_progress(db, self.player_id, prerequisite.id).await?;
            if progress.is_none_or(|progress| progress.step_id != -1) {
                self.finish(db, prerequisite.id, completion_step).await?;
            }
        }

        for teaching in tables
            .teaching_summon
            .iter()
            .filter(|teaching| teaching.grant_guide_id == guide.id)
        {
            let progress = guides::get_guide_progress(db, self.player_id, teaching.id).await?;
            if progress
                .as_ref()
                .is_some_and(|progress| progress.step_id == -1)
            {
                continue;
            }
            if progress.is_none() {
                guides::update_guide_progress(
                    db,
                    self.player_id,
                    teaching.id,
                    teaching.previous_step_id,
                )
                .await?;
            }
            if progress
                .as_ref()
                .map(|progress| progress.step_id)
                .unwrap_or(teaching.previous_step_id)
                == teaching.previous_step_id
            {
                crate::summon::SummonManager::new(self.player_id)
                    .summon(
                        db,
                        teaching.pool_id,
                        Some(teaching.id),
                        Some(teaching.step_id),
                        1,
                    )
                    .await?;
            }
            let completion_step =
                apply_guide_world_progress(db, self.player_id, teaching.id).await?;
            self.finish(db, teaching.id, completion_step).await?;
        }
        Ok(())
    }

    pub async fn finish(
        &self,
        db: &SqlitePool,
        guide_id: i32,
        step_id: i32,
    ) -> Result<GuideCompletion, AppError> {
        if config::configs::get()
            .guide
            .get(guide_id)
            .is_none_or(|guide| guide.is_online == 0)
        {
            return Err(AppError::InvalidRequest);
        }
        let required_story = if step_id == 0 {
            None
        } else {
            let step = config::configs::get()
                .guide_step
                .iter()
                .find(|step| step.id == guide_id && step.step_id == step_id)
                .ok_or(AppError::InvalidRequest)?;
            story_requirement(&step.action)
        };
        if let Some(story_id) = required_story
            && !stories::is_story_finished(db, self.player_id, story_id).await?
        {
            return Err(AppError::InvalidRequest);
        }

        let previous = guides::get_guide_progress(db, self.player_id, guide_id).await?;
        let first_completion = previous
            .as_ref()
            .is_none_or(|progress| progress.step_id != -1 && progress.step_id < step_id);
        let stored_step_id = stored_step_after(guide_id, step_id);
        let hero_id = required_story.and_then(|_| hero_reward_after_step(guide_id, step_id));
        let common_group = if hero_id.is_some() {
            Some(
                hero_groups::get_hero_groups_common(db, self.player_id)
                    .await?
                    .into_iter()
                    .next()
                    .ok_or(AppError::InvalidRequest)?,
            )
        } else {
            None
        };
        let heroes = UserHeroModel::new(self.player_id, db.clone());
        let should_grant = if first_completion {
            match hero_id {
                Some(hero_id) => !heroes.has_hero(hero_id).await?,
                None => false,
            }
        } else {
            false
        };
        let mut tx = db.begin().await?;
        let (rewards, material_changes) = if first_completion {
            let mut rewards = teaching_rewards(guide_id, step_id);
            if should_grant {
                rewards.heroes.push((hero_id.unwrap(), 1));
            }
            let material_changes = rewards.material_changes();
            (
                reward::apply_in_transaction(&mut tx, db, self.player_id, rewards).await?,
                material_changes,
            )
        } else {
            Default::default()
        };
        if !guides::update_guide_progress_in_transaction(
            &mut tx,
            self.player_id,
            guide_id,
            previous.as_ref().map(|progress| progress.step_id),
            stored_step_id,
        )
        .await?
        {
            return Err(AppError::InvalidRequest);
        }
        let group_snapshot = if let (Some(hero_id), Some(mut group)) = (hero_id, common_group) {
            let hero_uid = heroes.hero_uid_in_transaction(&mut tx, hero_id).await?;
            group.hero_list = vec![hero_uid];
            hero_group_snapshots::save_common_group_snapshot_in_transaction(
                &mut tx,
                self.player_id,
                HeroGroupSnapshotType::Common.id(),
                &group,
            )
            .await?;
            Some(UpdateHeroGroupSnapshotPush {
                snapshot_id: Some(HeroGroupSnapshotType::Common.id()),
                snapshot_sub_id: Some(group.group_id),
                group_info: Some(group.into()),
            })
        } else {
            None
        };
        tx.commit().await?;
        Ok(GuideCompletion {
            reply: FinishGuideReply {},
            guide_info: GuideInfo {
                guide_id,
                step_id: stored_step_id,
            },
            rewards,
            material_changes,
            group_snapshot,
        })
    }
}

#[cfg(test)]
mod test;
