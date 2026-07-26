use crate::{error::AppError, reward, types::hero_group_snapshot_type::HeroGroupSnapshotType};
use database::{
    db::game::{guides, hero_group_snapshots, hero_groups, stories},
    models::game::heros::UserHeroModel,
};
use sonettobuf::{FinishGuideReply, GetGuideInfoReply, GuideInfo, UpdateHeroGroupSnapshotPush};
use sqlx::SqlitePool;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum GuideActionKind {
    OpenView = 105,
    WaitForStory = 204,
}

impl GuideActionKind {
    fn tag(self) -> &'static str {
        match self {
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

pub async fn get_guide_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetGuideInfoReply, AppError> {
    let guide_infos = guides::get_all_guide_progress(db, player_id).await?;

    Ok(GetGuideInfoReply {
        guide_infos: guide_infos.into_iter().map(Into::into).collect(),
    })
}

pub async fn finish_guide(
    db: &SqlitePool,
    player_id: i64,
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
        && !stories::is_story_finished(db, player_id, story_id).await?
    {
        return Err(AppError::InvalidRequest);
    }

    let previous = guides::get_guide_progress(db, player_id, guide_id).await?;
    let first_completion = previous
        .as_ref()
        .is_none_or(|progress| progress.step_id != -1 && progress.step_id < step_id);
    let stored_step_id = stored_step_after(guide_id, step_id);
    let hero_id = required_story.and_then(|_| hero_reward_after_step(guide_id, step_id));
    let common_group = if hero_id.is_some() {
        Some(
            hero_groups::get_hero_groups_common(db, player_id)
                .await?
                .into_iter()
                .next()
                .ok_or(AppError::InvalidRequest)?,
        )
    } else {
        None
    };
    let heroes = UserHeroModel::new(player_id, db.clone());
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
            reward::apply_in_transaction(&mut tx, db, player_id, rewards).await?,
            material_changes,
        )
    } else {
        Default::default()
    };
    if !guides::update_guide_progress_in_transaction(
        &mut tx,
        player_id,
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
            player_id,
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

#[cfg(test)]
mod test;
