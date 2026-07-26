use crate::{error::AppError, reward};
use database::{
    db::{
        game::{guides, summon},
        user::account,
    },
    models::game::{currencies::UserCurrencyModel, heros::UserHeroModel, items::UserItemModel},
};
use rand::{Rng, prelude::IndexedRandom};
use sonettobuf::{
    ChooseEnhancedPoolHeroReply, ChooseMultiUpHeroReply, EndActivityPush, GetSummonInfoReply,
    GetSummonProgressRewardsReply, GuideInfo, PopUpRecommendWindowReply, SummonQueryTokenReply,
    SummonReply, SummonResult,
};
use sqlx::SqlitePool;
mod commands;
mod parse;
mod pool;

pub use commands::*;
use parse::{choose_weighted, parse_ids, parse_up_heroes, parse_weighted};
pub(crate) use pool::build_gacha_pool;
use pool::{GachaResult, GachaRules, GachaState, SummonType};

#[cfg(test)]
mod test;
