use super::*;
use crate::engine::manager::card::CardSetup;
use sonettobuf::{
    BeginRoundOper, BeginRoundRequest, FightEntityInfo, FightTeam, UseClothSkillRequest,
};

mod auto_battle;
mod cards;
mod cloth;
mod core;
mod qte;
mod rounds;
mod terminal;
