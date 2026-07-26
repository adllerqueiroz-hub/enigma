use crate::{
    error::AppError,
    reward,
    types::{activity_id::ActivityId, copost_const_id::CopostConstId},
};
use chrono::{NaiveDateTime, TimeZone, Utc};
use database::db::game::{
    activity_state::{self, ActivityStateKind, ActivityStateSet},
    activity101,
};
use serde::{Deserialize, Serialize};
use sonettobuf::{
    AcceptAct186SpBonusReply, Act101Info, Act101SpInfo, Act104EpisodeNo, Act104PreSummaryNo,
    Act104RetailNo, Act104SpecialNo, Act104TrialNo, Act123EpisodeNo, Act123RetailNo, Act123StageNo,
    Act125Episode, Act146Episode, Act146EpisodeBonusReply, Act160FinishMissionReply,
    Act160GetInfoReply, Act160MissionInfo, Act165GainMilestoneRewardReply,
    Act165GenerateEndingReply, Act165GetInfoReply, Act165ModifyKeywordReply, Act165RestartReply,
    Act165StoryInfo, Act172Info, Act186GameInfo, Act186Info, Act186LikeInfo, Act186TaskInfo,
    Act205GetGameInfoReply, Act205GetInfoReply, Act206ChooseDirectionReply, Act206GetInfoReply,
    Act208BonusNo, Act208ReceiveBonusReply, Act212BonusNo, Act212InfoNo, Act212ReceiveBonusReply,
    Act218FinishGameReply, Act221SummonReply, Act228FlipGridGridReply, Act228GetFinalBonusReply,
    Act228Info, ActivityInfo, ActivityNewStageReadReply, EndingInfo, FinishAct125EpisodeReply,
    FinishAct146EpisodeReply, Get101BonusReply, Get101InfosReply, Get101SpBonusReply,
    Get104InfosReply, Get123InfosReply, Get136InfoReply, Get152InfoReply, Get153InfosReply,
    Get154InfosReply, Get158InfosReply, Get166InfosReply, Get196InfoReply, Get197InfoReply,
    Get199InfoReply, Get218InfoReply, Get221InfoReply, GetAct125InfosReply, GetAct146InfosReply,
    GetAct172InfoReply, GetAct186InfoReply, GetAct186SpBonusInfoReply, GetAct189InfoReply,
    GetAct189OnceBonusReply, GetAct208InfoReply, GetAct209InfoReply, GetAct212InfoReply,
    GetAct216InfoReply, GetAct225InfoReply, GetAct228InfoReply, GetActivityInfosReply,
    GetActivityInfosWithParamReply, MarkActivity104StoryReply, MarkEpisodeAfterStoryReply,
    MarkPopSummaryReply, StepInfo, UnlockPermanentReply,
};
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};

const PERMANENT_END_MS: u64 = 2_145_934_800_000;

pub(crate) mod act101;
pub(crate) mod act104;
pub(crate) mod act123;
pub(crate) mod act125;
pub(crate) mod act136;
pub(crate) mod act146;
pub(crate) mod act152;
pub(crate) mod act154;
pub(crate) mod act158;
pub(crate) mod act160;
pub(crate) mod act165;
pub(crate) mod act166;
pub(crate) mod act172;
pub(crate) mod act186;
pub(crate) mod act189;
pub(crate) mod act196;
pub(crate) mod act197;
pub(crate) mod act198;
pub(crate) mod act199;
pub(crate) mod act205;
pub(crate) mod act206;
pub(crate) mod act208;
pub(crate) mod act209;
pub(crate) mod act212;
pub(crate) mod act216;
pub(crate) mod act217;
pub(crate) mod act218;
pub(crate) mod act221;
pub(crate) mod act225;
pub(crate) mod act228;
pub(crate) mod act229;

pub use act101::{get101_bonus, get101_infos, get101_sp_bonus};
pub use act104::{
    act104_infos, mark_activity104_story, mark_episode_after_story, mark_pop_summary,
};
pub use act123::{act123_infos, act153_infos};
pub use act125::{act125_infos, finish_act125_episode};
pub use act136::{act136_info, act136_select};
pub use act146::{act146_episode_bonus, act146_infos, finish_act146_episode};
pub use act152::{accept_act152_present, act152_info};
pub use act154::{act154_infos, answer154_puzzle};
pub use act158::act158_infos;
pub use act160::{act160_get_info, finish_act160_mission};
pub use act165::{
    act165_gain_milestone_reward, act165_generate_ending, act165_get_info, act165_modify_keyword,
    act165_restart,
};
pub use act166::act166_infos;
pub use act172::act172_info;
pub use act186::{accept_act186_sp_bonus, act186_info, get_act186_sp_bonus_info};
pub use act189::{act189_info, get_act189_once_bonus};
pub use act196::{act196_gain, act196_info};
pub use act197::{act197_explore, act197_info, act197_rummage};
pub use act198::act198_gain;
pub use act199::{act199_gain, act199_info};
pub use act205::{act205_finish_game, act205_get_game_info, act205_get_info};
pub use act206::{act206_choose_direction, act206_get_bonus, act206_get_info};
pub use act208::{act208_info, receive_act208_bonus};
pub use act209::act209_info;
pub use act212::{act212_info, receive_act212_bonus};
pub use act216::{act216_info, finish_act216_task, get_act216_once_bonus};
pub use act217::act217_infos;
pub use act218::{accept_act218_reward, act218_info, finish_act218_game};
pub use act221::{act221_info, act221_select, act221_summon};
pub use act225::act225_info;
pub use act228::{act228_flip_grid, act228_get_final_bonus, act228_info};
pub use act229::act229_info;

#[derive(Clone, Debug)]
pub struct ActivityManager {
    player_id: i64,
    states: HashMap<(i32, i32), activity_state::ActivityStates>,
}

impl ActivityManager {
    pub fn new(player_id: i64) -> Self {
        Self {
            player_id,
            states: HashMap::new(),
        }
    }

    pub async fn get101_infos(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Get101InfosReply, AppError> {
        let reply = get101_infos(db, self.player_id, activity_id).await?;
        let activity_id = reply.activity_id.unwrap_or_else(latest_act101_activity_id);
        self.refresh_states(db, activity_id, ActivityStateKind::Act101Day)
            .await?;
        self.refresh_states(db, activity_id, ActivityStateKind::Act101Once)
            .await?;
        self.refresh_states(db, activity_id, ActivityStateKind::Act101SpBonus)
            .await?;
        Ok(reply)
    }

    pub async fn get101_bonus(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        day_id: Option<u32>,
    ) -> Result<act101::Activity101Claim, AppError> {
        let claim = get101_bonus(db, self.player_id, activity_id, day_id).await?;
        let activity_id = claim
            .reply
            .activity_id
            .unwrap_or_else(latest_act101_activity_id);
        self.refresh_states(db, activity_id, ActivityStateKind::Act101Day)
            .await?;
        self.refresh_states(db, activity_id, ActivityStateKind::Act101Once)
            .await?;
        Ok(claim)
    }

    pub async fn get101_sp_bonus(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        id: Option<i32>,
    ) -> Result<act101::Activity101SpClaim, AppError> {
        let claim = get101_sp_bonus(db, self.player_id, activity_id, id).await?;
        let activity_id = claim
            .reply
            .activity_id
            .unwrap_or_else(latest_act101_activity_id);
        self.refresh_states(db, activity_id, ActivityStateKind::Act101SpBonus)
            .await?;
        Ok(claim)
    }

    pub async fn act104_infos(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Get104InfosReply, AppError> {
        let reply = act104_infos(db, self.player_id, activity_id).await?;
        if let Some(activity_id) = reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act104Episode)
                .await?;
            self.refresh_states(db, activity_id, ActivityStateKind::Act104Special)
                .await?;
            self.refresh_states(db, activity_id, ActivityStateKind::Act104AfterStory)
                .await?;
            self.refresh_states(db, activity_id, ActivityStateKind::Act104Story)
                .await?;
            self.refresh_states(db, activity_id, ActivityStateKind::Act104PopSummary)
                .await?;
        }
        Ok(reply)
    }

    pub async fn mark_activity104_story(
        &mut self,
        db: &SqlitePool,
        activity_id: i32,
    ) -> Result<MarkActivity104StoryReply, AppError> {
        let reply = mark_activity104_story(db, self.player_id, activity_id).await?;
        self.refresh_states(db, activity_id, ActivityStateKind::Act104Story)
            .await?;
        Ok(reply)
    }

    pub async fn mark_pop_summary(
        &mut self,
        db: &SqlitePool,
        activity_id: i32,
    ) -> Result<MarkPopSummaryReply, AppError> {
        let reply = mark_pop_summary(db, self.player_id, activity_id).await?;
        self.refresh_states(db, activity_id, ActivityStateKind::Act104PopSummary)
            .await?;
        Ok(reply)
    }

    pub async fn mark_episode_after_story(
        &mut self,
        db: &SqlitePool,
        activity_id: i32,
        layer: i32,
    ) -> Result<MarkEpisodeAfterStoryReply, AppError> {
        let reply = mark_episode_after_story(db, self.player_id, activity_id, layer).await?;
        self.refresh_states(db, activity_id, ActivityStateKind::Act104AfterStory)
            .await?;
        Ok(reply)
    }

    pub async fn get_act186_sp_bonus_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        act186_activity_id: Option<i32>,
    ) -> Result<GetAct186SpBonusInfoReply, AppError> {
        let reply =
            get_act186_sp_bonus_info(db, self.player_id, activity_id, act186_activity_id).await?;
        if let Some(activity_id) = reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act186SpBonus)
                .await?;
        }
        Ok(reply)
    }

    pub async fn act186_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<GetAct186InfoReply, AppError> {
        let reply = act186_info(db, self.player_id, activity_id).await?;
        if let Some(activity_id) = reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act186Task)
                .await?;
        }
        Ok(reply)
    }

    pub async fn accept_act186_sp_bonus(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        act186_activity_id: Option<i32>,
    ) -> Result<AcceptAct186SpBonusReply, AppError> {
        let reply =
            accept_act186_sp_bonus(db, self.player_id, activity_id, act186_activity_id).await?;
        if let Some(activity_id) = reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act186SpBonus)
                .await?;
        }
        Ok(reply)
    }

    pub async fn act189_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<GetAct189InfoReply, AppError> {
        let reply = act189_info(db, self.player_id, activity_id).await?;
        if let Some(activity_id) = reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act189OnceBonus)
                .await?;
        }
        Ok(reply)
    }

    pub async fn get_act189_once_bonus(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<act189::Act189OnceBonusClaim, AppError> {
        let claim = get_act189_once_bonus(db, self.player_id, activity_id).await?;
        if let Some(activity_id) = claim.reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act189OnceBonus)
                .await?;
        }
        Ok(claim)
    }

    pub async fn act199_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Get199InfoReply, AppError> {
        act199_info(db, self.player_id, activity_id).await
    }

    pub async fn act199_gain(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        hero_id: Option<i32>,
    ) -> Result<act199::Act199GainClaim, AppError> {
        act199_gain(db, self.player_id, activity_id, hero_id).await
    }

    pub async fn act196_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Get196InfoReply, AppError> {
        act196_info(db, self.player_id, activity_id).await
    }

    pub async fn act196_gain(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        id: Option<i32>,
    ) -> Result<act196::Act196Claim, AppError> {
        act196_gain(db, self.player_id, activity_id, id).await
    }

    pub async fn act197_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Get197InfoReply, AppError> {
        act197_info(db, self.player_id, activity_id).await
    }

    pub async fn act197_rummage(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        pool_id: Option<i32>,
    ) -> Result<act197::Act197Claim, AppError> {
        act197_rummage(db, self.player_id, activity_id, pool_id).await
    }

    pub async fn act197_explore(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        find_type: Option<i32>,
    ) -> Result<act197::Act197Explore, AppError> {
        act197_explore(db, self.player_id, activity_id, find_type).await
    }

    pub async fn act198_gain(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<act198::Act198Claim, AppError> {
        act198_gain(db, self.player_id, activity_id).await
    }

    pub async fn act205_get_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Act205GetInfoReply, AppError> {
        act205_get_info(db, self.player_id, activity_id).await
    }

    pub async fn act205_get_game_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Act205GetGameInfoReply, AppError> {
        act205_get_game_info(db, self.player_id, activity_id).await
    }

    pub async fn act205_finish_game(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        game_type: Option<i32>,
        game_info: Option<String>,
        reward_id: Option<i32>,
    ) -> Result<act205::Act205Claim, AppError> {
        act205_finish_game(
            db,
            self.player_id,
            activity_id,
            game_type,
            game_info,
            reward_id,
        )
        .await
    }

    pub async fn act206_get_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Act206GetInfoReply, AppError> {
        act206_get_info(db, self.player_id, activity_id).await
    }

    pub async fn act206_choose_direction(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        direction_id: Option<i32>,
    ) -> Result<Act206ChooseDirectionReply, AppError> {
        act206_choose_direction(db, self.player_id, activity_id, direction_id).await
    }

    pub async fn act206_get_bonus(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<act206::Act206Claim, AppError> {
        act206_get_bonus(db, self.player_id, activity_id).await
    }

    pub async fn act221_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Get221InfoReply, AppError> {
        act221_info(db, self.player_id, activity_id).await
    }

    pub async fn act221_summon(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Act221SummonReply, AppError> {
        act221_summon(db, self.player_id, activity_id).await
    }

    pub async fn act221_select(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        select_index: Option<i32>,
    ) -> Result<act221::Act221Claim, AppError> {
        act221_select(db, self.player_id, activity_id, select_index).await
    }

    pub async fn act125_infos(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<GetAct125InfosReply, AppError> {
        let reply = act125_infos(db, self.player_id, activity_id).await?;
        let activity_id = reply.activity_id.unwrap_or_else(default_act125_activity_id);
        self.refresh_states(db, activity_id, ActivityStateKind::Act125Episode)
            .await?;
        Ok(reply)
    }

    pub async fn finish_act125_episode(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        episode_id: Option<i32>,
        target_frequency: Option<i32>,
    ) -> Result<act125::Act125Claim, AppError> {
        let reply = finish_act125_episode(
            db,
            self.player_id,
            activity_id,
            episode_id,
            target_frequency,
        )
        .await?;
        let activity_id = reply
            .reply
            .activity_id
            .unwrap_or_else(default_act125_activity_id);
        self.refresh_states(db, activity_id, ActivityStateKind::Act125Episode)
            .await?;
        Ok(reply)
    }

    pub async fn act136_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Get136InfoReply, AppError> {
        act136_info(db, self.player_id, activity_id).await
    }

    pub async fn act136_select(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        hero_id: Option<i32>,
    ) -> Result<act136::Act136SelectClaim, AppError> {
        act136_select(db, self.player_id, activity_id, hero_id).await
    }

    pub async fn act146_infos(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<GetAct146InfosReply, AppError> {
        let reply = act146_infos(db, self.player_id, activity_id).await?;
        if let Some(activity_id) = reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act146Episode)
                .await?;
        }
        Ok(reply)
    }

    pub async fn finish_act146_episode(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        episode_id: Option<i32>,
    ) -> Result<FinishAct146EpisodeReply, AppError> {
        let reply = finish_act146_episode(db, self.player_id, activity_id, episode_id).await?;
        if let Some(activity_id) = reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act146Episode)
                .await?;
        }
        Ok(reply)
    }

    pub async fn act146_episode_bonus(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        episode_id: Option<i32>,
    ) -> Result<act146::Act146Claim, AppError> {
        let claim = act146_episode_bonus(db, self.player_id, activity_id, episode_id).await?;
        if let Some(activity_id) = claim.reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act146Episode)
                .await?;
        }
        Ok(claim)
    }

    pub async fn act152_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Get152InfoReply, AppError> {
        act152_info(db, self.player_id, activity_id).await
    }

    pub async fn accept_act152_present(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        present_id: Option<i32>,
    ) -> Result<act152::Act152PresentClaim, AppError> {
        accept_act152_present(db, self.player_id, activity_id, present_id).await
    }

    pub async fn act154_infos(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Get154InfosReply, AppError> {
        act154_infos(db, self.player_id, activity_id).await
    }

    pub async fn answer154_puzzle(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        puzzle_id: Option<u32>,
        option_id: Option<u32>,
    ) -> Result<act154::Act154Claim, AppError> {
        answer154_puzzle(db, self.player_id, activity_id, puzzle_id, option_id).await
    }

    pub fn act158_infos(&self, activity_id: Option<i32>) -> Get158InfosReply {
        act158_infos(activity_id)
    }

    pub async fn act160_get_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Act160GetInfoReply, AppError> {
        let reply = act160_get_info(db, self.player_id, activity_id).await?;
        let activity_id = reply.activity_id.unwrap_or_else(latest_act160_activity_id);
        self.refresh_states(db, activity_id, ActivityStateKind::Act160Mission)
            .await?;
        Ok(reply)
    }

    pub async fn act172_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<GetAct172InfoReply, AppError> {
        let reply = act172_info(db, self.player_id, activity_id).await?;
        if let Some(activity_id) = reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act172UseItemTask)
                .await?;
        }
        Ok(reply)
    }

    pub async fn finish_act160_mission(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        id: Option<i32>,
    ) -> Result<act160::Act160Claim, AppError> {
        let claim = finish_act160_mission(db, self.player_id, activity_id, id).await?;
        let activity_id = claim
            .reply
            .activity_id
            .unwrap_or_else(latest_act160_activity_id);
        self.refresh_states(db, activity_id, ActivityStateKind::Act160Mission)
            .await?;
        Ok(claim)
    }

    pub async fn act165_get_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Act165GetInfoReply, AppError> {
        let reply = act165_get_info(db, self.player_id, activity_id).await?;
        let activity_id = reply.activity_id.unwrap_or_else(latest_act165_activity_id);
        self.refresh_states(db, activity_id, ActivityStateKind::Act165Story)
            .await?;
        Ok(reply)
    }

    pub async fn act165_modify_keyword(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        story_id: Option<i32>,
        keyword_ids: Vec<i32>,
    ) -> Result<Act165ModifyKeywordReply, AppError> {
        let reply =
            act165_modify_keyword(db, self.player_id, activity_id, story_id, keyword_ids).await?;
        let activity_id = reply.activity_id.unwrap_or_else(latest_act165_activity_id);
        self.refresh_states(db, activity_id, ActivityStateKind::Act165Story)
            .await?;
        Ok(reply)
    }

    pub async fn act165_generate_ending(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        story_id: Option<i32>,
    ) -> Result<Act165GenerateEndingReply, AppError> {
        let reply = act165_generate_ending(db, self.player_id, activity_id, story_id).await?;
        let activity_id = reply.activity_id.unwrap_or_else(latest_act165_activity_id);
        self.refresh_states(db, activity_id, ActivityStateKind::Act165Story)
            .await?;
        Ok(reply)
    }

    pub async fn act165_restart(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        story_id: Option<i32>,
        step_id: Option<i32>,
    ) -> Result<Act165RestartReply, AppError> {
        let reply = act165_restart(db, self.player_id, activity_id, story_id, step_id).await?;
        let activity_id = reply.activity_id.unwrap_or_else(latest_act165_activity_id);
        self.refresh_states(db, activity_id, ActivityStateKind::Act165Story)
            .await?;
        Ok(reply)
    }

    pub async fn act165_gain_milestone_reward(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        story_id: Option<i32>,
    ) -> Result<act165::Act165RewardClaim, AppError> {
        let claim = act165_gain_milestone_reward(db, self.player_id, activity_id, story_id).await?;
        let activity_id = claim
            .reply
            .activity_id
            .unwrap_or_else(latest_act165_activity_id);
        self.refresh_states(db, activity_id, ActivityStateKind::Act165Story)
            .await?;
        Ok(claim)
    }

    pub fn act166_infos(&self, activity_id: Option<i32>) -> Get166InfosReply {
        act166_infos(activity_id)
    }

    pub async fn act208_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<GetAct208InfoReply, AppError> {
        let reply = act208_info(db, self.player_id, activity_id).await?;
        if let Some(activity_id) = reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act208Bonus)
                .await?;
        }
        Ok(reply)
    }

    pub async fn receive_act208_bonus(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        id: Option<i32>,
    ) -> Result<act208::Act208Claim, AppError> {
        let claim = receive_act208_bonus(db, self.player_id, activity_id, id).await?;
        if let Some(activity_id) = claim.reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act208Bonus)
                .await?;
        }
        Ok(claim)
    }

    pub async fn act209_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<GetAct209InfoReply, AppError> {
        let reply = act209_info(db, self.player_id, activity_id).await?;
        if let Some(activity_id) = reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act209Layer)
                .await?;
        }
        Ok(reply)
    }

    pub async fn act212_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<GetAct212InfoReply, AppError> {
        let reply = act212_info(db, self.player_id, activity_id).await?;
        if let Some(info) = &reply.act212_info
            && let Some(activity_id) = info.activity_id
        {
            self.refresh_states(db, activity_id, ActivityStateKind::Act212Bonus)
                .await?;
        }
        Ok(reply)
    }

    pub async fn receive_act212_bonus(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        id: Option<i32>,
    ) -> Result<act212::Act212Claim, AppError> {
        let claim = receive_act212_bonus(db, self.player_id, activity_id, id).await?;
        if let Some(activity_id) = claim.reply.activity_id {
            self.refresh_states(db, activity_id, ActivityStateKind::Act212Bonus)
                .await?;
        }
        Ok(claim)
    }

    pub async fn act216_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<GetAct216InfoReply, AppError> {
        act216_info(db, self.player_id, activity_id).await
    }

    pub async fn finish_act216_task(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        task_id: Option<i32>,
    ) -> Result<act216::Act216TaskClaim, AppError> {
        finish_act216_task(db, self.player_id, activity_id, task_id).await
    }

    pub async fn get_act216_once_bonus(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<act216::Act216OnceBonusClaim, AppError> {
        get_act216_once_bonus(db, self.player_id, activity_id).await
    }

    pub async fn act225_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<GetAct225InfoReply, AppError> {
        act225_info(db, self.player_id, activity_id).await
    }

    pub async fn act218_info(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<Get218InfoReply, AppError> {
        act218_info(db, self.player_id, activity_id).await
    }

    pub async fn finish_act218_game(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
        result: Option<i32>,
        game_record: Option<String>,
    ) -> Result<Act218FinishGameReply, AppError> {
        finish_act218_game(db, self.player_id, activity_id, result, game_record).await
    }

    pub async fn accept_act218_reward(
        &mut self,
        db: &SqlitePool,
        activity_id: Option<i32>,
    ) -> Result<act218::Act218RewardClaim, AppError> {
        accept_act218_reward(db, self.player_id, activity_id).await
    }

    async fn refresh_states(
        &mut self,
        db: &SqlitePool,
        activity_id: i32,
        kind: ActivityStateKind,
    ) -> Result<(), AppError> {
        let states = activity_state::get(db, self.player_id, activity_id, kind).await?;
        self.states.insert((activity_id, kind.id()), states);
        Ok(())
    }
}

pub async fn activity_infos(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetActivityInfosReply, AppError> {
    let mut infos = catalog_infos();
    apply_bp_activity(&mut infos);
    apply_act125_activity(&mut infos);
    apply_activity_state(db, player_id, &mut infos).await?;

    Ok(GetActivityInfosReply {
        activity_infos: infos,
    })
}

pub async fn activity_infos_with_param(
    db: &SqlitePool,
    player_id: i64,
    activity_ids: &[i32],
) -> Result<GetActivityInfosWithParamReply, AppError> {
    let requested = activity_ids.iter().copied().collect::<HashSet<_>>();
    let mut infos = catalog_infos();
    apply_bp_activity(&mut infos);
    apply_act125_activity(&mut infos);
    if !requested.is_empty() {
        infos.retain(|info| info.id.is_some_and(|id| requested.contains(&(id as i32))));
    }
    apply_activity_state(db, player_id, &mut infos).await?;

    Ok(GetActivityInfosWithParamReply {
        activity_infos: infos,
    })
}

pub async fn activity_new_stage_read(
    db: &SqlitePool,
    player_id: i64,
    mut ids: Vec<u32>,
) -> Result<ActivityNewStageReadReply, AppError> {
    ids.sort_unstable();
    ids.dedup();

    for id in &ids {
        activity_state::set_activity_flag(
            db,
            player_id,
            *id as i32,
            ActivityStateKind::ActivityNewStage,
            false,
        )
        .await?;
    }

    Ok(ActivityNewStageReadReply { id: ids })
}

pub async fn unlock_permanent(
    db: &SqlitePool,
    player_id: i64,
    id: Option<u32>,
) -> Result<UnlockPermanentReply, AppError> {
    if let Some(id) = id {
        activity_state::set_activity_flag(
            db,
            player_id,
            id as i32,
            ActivityStateKind::ActivityPermanentUnlock,
            true,
        )
        .await?;
    }

    Ok(UnlockPermanentReply { id })
}

async fn apply_activity_state(
    db: &SqlitePool,
    player_id: i64,
    infos: &mut [ActivityInfo],
) -> Result<(), AppError> {
    let new_stage =
        activity_state::get_activity_flags(db, player_id, ActivityStateKind::ActivityNewStage)
            .await?;
    let permanent_unlock = activity_state::get_activity_flags(
        db,
        player_id,
        ActivityStateKind::ActivityPermanentUnlock,
    )
    .await?;

    for info in infos {
        let Some(activity_id) = info.id.map(|id| id as i32) else {
            continue;
        };

        info.is_new_stage = Some(new_stage.contains(&activity_id));
        info.is_unlock =
            Some(is_unlocked_by_default(activity_id) || permanent_unlock.contains(&activity_id));
    }

    Ok(())
}

fn catalog_infos() -> Vec<ActivityInfo> {
    config::configs::get()
        .activity
        .iter()
        .map(|activity| activity_info(activity.id))
        .collect()
}

fn is_open(open_id: i32) -> bool {
    open_id == 0
        || config::configs::get()
            .open
            .get(open_id)
            .is_some_and(|open| open.is_online != 0)
}

fn apply_bp_activity(infos: &mut Vec<ActivityInfo>) {
    let Some(bp) = database::db::game::tasks::current_battle_pass() else {
        return;
    };
    if bp.activity_id <= 0 {
        return;
    }

    if let Some(info) = infos
        .iter_mut()
        .find(|info| info.id == Some(bp.activity_id as u32))
    {
        *info = activity_info(bp.activity_id);
    } else {
        infos.push(activity_info(bp.activity_id));
    }
}

fn apply_act125_activity(infos: &mut Vec<ActivityInfo>) {
    for activity_id in act125_activity_ids() {
        if infos.iter().all(|info| info.id != Some(activity_id as u32)) {
            infos.push(activity_info(activity_id));
        }
    }
}

fn default_act125_activity_id() -> i32 {
    act125_activity_ids()
        .next()
        .unwrap_or(ActivityId::V3a6CultivationDestiny.id())
}

fn act125_activity_ids() -> impl Iterator<Item = i32> {
    let tables = config::configs::get();

    ActivityId::ACT125
        .iter()
        .map(|activity_id| activity_id.id())
        .filter(move |activity_id| {
            tables
                .activity
                .get(*activity_id)
                .is_some_and(|activity| activity.type_id == 125 && is_open(activity.open_id))
        })
}

fn latest_act101_activity_id() -> i32 {
    latest_config_activity_id(
        config::configs::get()
            .activity101
            .iter()
            .map(|row| row.activity_id),
        ActivityId::SilverLitNight,
    )
}

fn latest_act160_activity_id() -> i32 {
    latest_config_activity_id(
        config::configs::get()
            .activity160_mission
            .iter()
            .map(|row| row.activity_id),
        ActivityId::GiftOfTheBeginning,
    )
}

fn latest_act165_activity_id() -> i32 {
    latest_config_activity_id(
        config::configs::get()
            .activity165_story
            .iter()
            .map(|row| row.activity_id),
        ActivityId::StoryDeduction,
    )
}

fn latest_act212_activity_id() -> i32 {
    latest_config_activity_id(
        config::configs::get()
            .activity212_bonus
            .iter()
            .map(|row| row.activity_id),
        ActivityId::ManyFacesOfParis,
    )
}

fn latest_act228_activity_id() -> i32 {
    latest_config_activity_id(
        config::configs::get()
            .activity228
            .iter()
            .map(|row| row.activity_id),
        ActivityId::MoonlightGardening,
    )
}

fn latest_config_activity_id(ids: impl Iterator<Item = i32>, fallback: ActivityId) -> i32 {
    let mut candidates = ids.filter(|id| *id > 0).collect::<Vec<_>>();
    candidates.sort_unstable();
    candidates.dedup();

    candidates
        .iter()
        .rev()
        .copied()
        .find(|activity_id| {
            config::configs::get()
                .activity
                .get(*activity_id)
                .is_some_and(|activity| is_open(activity.open_id))
        })
        .or_else(|| candidates.last().copied())
        .unwrap_or_else(|| fallback.id())
}

fn activity_info(activity_id: i32) -> ActivityInfo {
    let (start_time, end_time) = activity_time_range(activity_id);
    let online = is_activity_online(activity_id);

    ActivityInfo {
        id: Some(activity_id as u32),
        start_time: Some(start_time),
        end_time: Some(end_time),
        online: Some(online),
        is_new_stage: Some(false),
        current_stage: Some(0),
        is_unlock: Some(is_unlocked_by_default(activity_id)),
        is_receive_all_bonus: Some(false),
    }
}

fn is_activity_online(activity_id: i32) -> bool {
    let is_active_catalog = ActivityId::ACTIVE_CATALOG
        .iter()
        .any(|active| active.id() == activity_id);
    let is_current_bp = database::db::game::tasks::current_battle_pass()
        .is_some_and(|bp| bp.activity_id == activity_id);

    (is_active_catalog || is_current_bp)
        && config::configs::get()
            .activity
            .get(activity_id)
            .is_none_or(|activity| is_open(activity.open_id))
}

fn activity_time_range(activity_id: i32) -> (u64, u64) {
    let is_permanent = config::configs::get()
        .activity
        .get(activity_id)
        .is_some_and(|activity| activity.is_retro_acitivity == 2);
    if is_permanent {
        return (0, PERMANENT_END_MS);
    }

    version_activity_time_range().unwrap_or((0, PERMANENT_END_MS))
}

fn version_activity_time_range() -> Option<(u64, u64)> {
    let tables = config::configs::get();
    let start = tables
        .copost_const
        .get(CopostConstId::ActivityStartTime.id())
        .and_then(|row| parse_config_time_millis(&row.value2));
    let end = tables
        .copost_const
        .get(CopostConstId::ActivityEndTime.id())
        .and_then(|row| parse_config_time_millis(&row.value2));

    start.zip(end).filter(|(start, end)| start < end)
}

fn parse_config_time_millis(value: &str) -> Option<u64> {
    let normalized = value.split_whitespace().collect::<Vec<_>>().join(" ");

    ["%Y-%m-%d %H:%M:%S", "%Y-%m-%d %-H:%M:%S"]
        .into_iter()
        .find_map(|format| NaiveDateTime::parse_from_str(&normalized, format).ok())
        .and_then(|time| {
            Utc.from_utc_datetime(&time)
                .timestamp_millis()
                .try_into()
                .ok()
        })
}

fn is_unlocked_by_default(activity_id: i32) -> bool {
    const PERMANENT_RETRO_TYPE: i32 = 2;

    match config::configs::get().activity.get(activity_id) {
        Some(activity) => activity.is_retro_acitivity != PERMANENT_RETRO_TYPE,
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use crate::types::activity_id::ActivityId;

    use super::{
        PERMANENT_END_MS, apply_act125_activity, catalog_infos, default_act125_activity_id,
        parse_config_time_millis, version_activity_time_range,
    };

    #[test]
    fn catalog_contains_every_configured_activity() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);

        let infos = catalog_infos();
        assert_eq!(infos.len(), config::configs::get().activity.len());
        assert!(
            config::configs::get()
                .activity
                .iter()
                .all(|activity| infos.iter().any(|info| info.id == Some(activity.id as u32)))
        );
    }

    #[test]
    fn old_lua_activity_ids_are_present_but_not_online() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);

        let infos = catalog_infos();
        let inactive_activity = infos
            .iter()
            .find(|info| info.id == Some(ActivityId::MoonlightGardening.id() as u32))
            .unwrap();
        let current_dungeon = infos
            .iter()
            .find(|info| info.id == Some(ActivityId::V3a6Dungeon.id() as u32))
            .unwrap();

        assert_eq!(inactive_activity.online, Some(false));
        assert_eq!(current_dungeon.online, Some(true));
    }

    #[test]
    fn permanent_end_matches_lua_millisecond_time_shape() {
        assert_eq!(PERMANENT_END_MS, 2_145_934_800_000);
    }

    #[test]
    fn activity_time_uses_copost_version_window() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);

        assert_eq!(
            version_activity_time_range(),
            Some((1_782_968_400_000, 1_784_782_799_000))
        );
    }

    #[test]
    fn parses_copost_time_with_extra_space() {
        assert_eq!(
            parse_config_time_millis("2026-07-23  4:59:59"),
            Some(1_784_782_799_000)
        );
    }

    #[test]
    fn empty_param_request_keeps_the_catalog_and_current_act125() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);

        let mut infos = catalog_infos();
        apply_act125_activity(&mut infos);

        assert!(infos.iter().any(|info| info.id.is_some()));
        assert_eq!(default_act125_activity_id(), 13610);
        assert!(infos.iter().any(|info| info.id == Some(13610)));
        assert!(
            infos
                .iter()
                .any(|info| info.id == Some(default_act125_activity_id() as u32))
        );
        assert_eq!(
            infos
                .iter()
                .find(|info| info.id == Some(13116))
                .and_then(|info| info.online),
            Some(false)
        );
        assert_eq!(
            infos
                .iter()
                .find(|info| info.id == Some(13612))
                .and_then(|info| info.online),
            Some(false)
        );
    }

    #[test]
    fn current_act125_claim_has_material_reward() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);
        let activity_id = default_act125_activity_id();
        let row = config::configs::get()
            .activity125
            .iter()
            .find(|row| row.activity_id == activity_id)
            .unwrap();

        assert!(
            !crate::reward::parse(&row.bonus)
                .material_changes()
                .is_empty()
        );
    }
}
