use crate::{error::AppError, logic::reward};
use chrono::{NaiveDateTime, TimeZone, Utc};
use database::db::game::{
    battle_pass,
    tasks::{self as task_db, TaskLoopType},
};
use sonettobuf::{
    BpBuyLevelReply, BpMarkFirstShowReply, BpScoreBonusInfo, GetBpBonusReply, GetBpInfoReply,
    GetSelfSelectBonusReply, RedDotInfo, Task,
};
use sqlx::SqlitePool;
use std::collections::HashMap;

const BP_BUY_LEVEL_COST_CONFIG_ID: i32 = 111;
mod info;
mod progression;
mod rewards;
mod tasks;

pub use info::*;
pub use progression::*;
pub use rewards::*;
pub use tasks::*;
use tasks::{bp_time_range, parse_bp_reward, score_bonus_info};

#[cfg(test)]
use info::bonus_red_dots_for_state;
#[cfg(test)]
use progression::level_purchase_cost;
#[cfg(test)]
use rewards::select_reward;
#[cfg(test)]
use tasks::{should_show_task_red_dot, task_tab_id};
#[cfg(test)]
mod test;
