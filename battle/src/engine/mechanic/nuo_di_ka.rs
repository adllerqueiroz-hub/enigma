use std::collections::HashMap;

use crate::engine::manager::hp::DamageEffectKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NuoDiKaHit {
    pub target_uid: i64,
    pub amount: i32,
    pub effect_kind: DamageEffectKind,
    pub mass: bool,
    pub hit_index: i32,
    pub points: i32,
    pub config_effect: i32,
    pub buff_act_id: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NuoDiKaState {
    pub points: i32,
    pub bloodtithe_consumed: i32,
    pub max_points: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NuoDiKaCommand {
    Set {
        owner_uid: i64,
        points: i32,
        bloodtithe_consumed: i32,
        max_points: i32,
    },
    Clear {
        owner_uid: i64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NuoDiKaChange {
    pub owner_uid: i64,
    pub before: NuoDiKaState,
    pub after: NuoDiKaState,
    pub active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NuoDiKaError {
    InvalidCommand,
    MissingChannel,
}

#[derive(Debug, Clone, Default)]
pub struct NuoDiKa {
    channel: HashMap<i64, NuoDiKaState>,
}

impl NuoDiKa {
    pub fn execute(&mut self, command: NuoDiKaCommand) -> Result<NuoDiKaChange, NuoDiKaError> {
        match command {
            NuoDiKaCommand::Set {
                owner_uid,
                points,
                bloodtithe_consumed,
                max_points,
            } => {
                if owner_uid == 0 || points <= 0 || bloodtithe_consumed <= 0 || max_points < points
                {
                    return Err(NuoDiKaError::InvalidCommand);
                }
                let before = self.channel.get(&owner_uid).copied().unwrap_or_default();
                let after = NuoDiKaState {
                    points,
                    bloodtithe_consumed,
                    max_points,
                };
                self.channel.insert(owner_uid, after);
                Ok(NuoDiKaChange {
                    owner_uid,
                    before,
                    after,
                    active: true,
                })
            }
            NuoDiKaCommand::Clear { owner_uid } => {
                if owner_uid == 0 {
                    return Err(NuoDiKaError::InvalidCommand);
                }
                let before = self.channel.remove(&owner_uid).unwrap_or_default();
                Ok(NuoDiKaChange {
                    owner_uid,
                    before,
                    after: NuoDiKaState::default(),
                    active: false,
                })
            }
        }
    }

    pub fn get(&self, owner_uid: i64) -> i32 {
        self.channel
            .get(&owner_uid)
            .map(|state| state.points)
            .unwrap_or_default()
    }

    pub fn bloodtithe_consumed(&self, owner_uid: i64) -> i32 {
        self.channel
            .get(&owner_uid)
            .map(|state| state.bloodtithe_consumed)
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_state_is_set_and_cleared_by_one_owner() {
        let mut channel = NuoDiKa::default();
        let set = channel
            .execute(NuoDiKaCommand::Set {
                owner_uid: 10,
                points: 9,
                bloodtithe_consumed: 18,
                max_points: 30,
            })
            .unwrap();

        assert!(set.active);
        assert_eq!(channel.get(10), 9);
        assert_eq!(channel.bloodtithe_consumed(10), 18);

        let cleared = channel
            .execute(NuoDiKaCommand::Clear { owner_uid: 10 })
            .unwrap();
        assert!(!cleared.active);
        assert_eq!(channel.get(10), 0);
        assert_eq!(
            channel
                .execute(NuoDiKaCommand::Clear { owner_uid: 10 })
                .unwrap()
                .before,
            NuoDiKaState::default()
        );
    }
}
