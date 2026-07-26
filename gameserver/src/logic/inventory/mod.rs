use crate::{error::AppError, logic::reward, types::red_dot_id::RedDotId};
use common::time::ServerTime;
use database::{
    db::game::{currencies, equipment as equipment_db, items, player_infos, red_dots},
    models::game::{
        heros::{HeroModel, InsightUpgrade, UserHeroModel},
        items::UserItemModel,
    },
};
use sonettobuf::{
    AutoUseExpirePowerItemReply, BuyPowerReply, CurrencyExchangeNo, ExchangeDiamondReply,
    ExchangeSameCurrencyReply, GetBuyPowerInfoReply, GetCurrencyListReply, GetItemListReply,
    M2qEntry, MarkReadSubType21Reply, PopExchangeSameCurrencyReply, PowerItem, UseInsightItemReply,
    UseItemReply, UsePowerItemInfo, UsePowerItemListReply, UsePowerItemReply,
};
use sqlx::SqlitePool;
use std::collections::BTreeMap;

#[repr(i32)]
enum ItemSubType {
    EquipmentLevelUp = 84,
}

#[repr(i32)]
enum EquipmentLevelEffectType {
    All = 1,
    Specify = 2,
}

impl EquipmentLevelEffectType {
    fn from_id(id: i32) -> Option<Self> {
        match id {
            id if id == Self::All as i32 => Some(Self::All),
            id if id == Self::Specify as i32 => Some(Self::Specify),
            _ => None,
        }
    }
}
mod currency;
mod equipment;
mod item;
mod power;

pub use currency::*;
use equipment::{target_item_rewards, use_equipment_level_item};
#[cfg(test)]
use item::item_rewards;
pub use item::*;
pub use power::*;

#[cfg(test)]
mod test;
