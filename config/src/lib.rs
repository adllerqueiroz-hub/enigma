// Generated modules are committed. Refresh them explicitly with `cargo run -p config_codegen`.
include!("../../config/configs/mod.rs");

// Handwritten semantic queries belong here, not in generated table files or callers.
mod activity_query;
mod battle_pass;
mod equipment;
mod hero;
mod player;
mod reward_query;
mod room;
mod scene;
mod summon_query;
mod task;
mod tower;

pub mod configs {
    pub use crate::{GameDB, get, init, try_get};
}
