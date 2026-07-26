mod bgm;
mod property;
mod settings;

#[derive(Clone, Copy, Debug)]
pub struct PreferenceManager {
    player_id: i64,
}

impl PreferenceManager {
    pub fn new(player_id: i64) -> Self {
        Self { player_id }
    }
}
