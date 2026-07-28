use crate::db::game::activity_state::ActivityStateKind;
use config::configs;
use sqlx::{Sqlite, SqlitePool, Transaction};

mod basic;
mod guide;
mod hero_group;
mod inventory;
mod misc;
mod room;
mod summon;
mod tasks;
mod tower_player;

use basic::{load_open_infos, load_player_info, load_player_state};
use guide::load_starter_guides;
use hero_group::{load_hero_touch_count, load_starter_hero_groups};
use inventory::{load_starter_currencies, load_starter_user_stats};
use misc::{
    load_activity_state, load_dungeon_reward_points, load_instruction_dungeon_info,
    load_starter_bgm, load_starter_settings, load_starter_system_state,
};
use room::load_starter_room;
use summon::load_starter_summon;
use tasks::load_starter_tasks;
use tower_player::load_tower_info;

pub async fn load_all_starter_data(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    tracing::info!("Loading all starter data for uid {uid} in a single transaction");

    ensure_game_data_loaded()?;
    let mut tx = pool.begin().await?;

    load_all_starter_data_tx(&mut tx, uid).await?;
    tx.commit().await?;

    tracing::info!("Finished loading all starter data for uid {uid}");
    Ok(())
}

pub async fn load_all_starter_data_tx(
    tx: &mut Transaction<'_, Sqlite>,
    uid: i64,
) -> sqlx::Result<()> {
    ensure_game_data_loaded()?;

    load_player_state(&mut *tx, uid).await?;
    load_player_info(&mut *tx, uid).await?;
    load_open_infos(&mut *tx, uid).await?;
    load_starter_guides(&mut *tx, uid).await?;
    load_hero_touch_count(&mut *tx, uid).await?;
    load_starter_currencies(&mut *tx, uid).await?;
    load_starter_user_stats(&mut *tx, uid).await?;
    load_starter_tasks(tx, uid).await?;
    load_starter_hero_groups(&mut *tx, uid).await?;
    load_starter_settings(&mut *tx, uid).await?;
    load_starter_system_state(&mut *tx, uid).await?;
    load_starter_room(&mut *tx, uid).await?;
    load_starter_summon(&mut *tx, uid).await?;
    load_tower_info(&mut *tx, uid).await?;
    load_dungeon_reward_points(&mut *tx, uid).await?;
    load_instruction_dungeon_info(&mut *tx, uid).await?;
    load_activity_state(&mut *tx, uid).await?;
    load_starter_bgm(&mut *tx, uid).await?;

    Ok(())
}

fn ensure_game_data_loaded() -> sqlx::Result<()> {
    if configs::try_get().is_some() {
        return Ok(());
    }

    let data_dir = common::excel_data_directory().to_string_lossy();
    match configs::init(&data_dir) {
        Ok(()) => Ok(()),
        Err(error) if configs::try_get().is_some() => {
            tracing::debug!("Game data was initialized concurrently: {error}");
            Ok(())
        }
        Err(error) => Err(sqlx::Error::Protocol(error.to_string())),
    }
}
