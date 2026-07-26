use std::collections::HashMap;

use crate::engine::skill::rule::CommandOrigin;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InjuryCounterKind {
    TeamInjury = 13,
}

impl InjuryCounterKind {
    pub const fn id(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InjuryRecord {
    pub origin: CommandOrigin,
    pub source_uid: i64,
    pub team_type: i32,
    pub injured_targets: Vec<i64>,
    pub counter_owner_uid: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InjuryCommand {
    RecordAction(InjuryRecord),
    Reset {
        origin: CommandOrigin,
        team_type: i32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InjuryChange {
    pub origin: CommandOrigin,
    pub source_uid: i64,
    pub team_type: i32,
    pub counter_owner_uid: i64,
    pub before: i32,
    pub after: i32,
}

#[derive(Debug, Clone, Default)]
pub struct InjuryManager {
    round_counts: HashMap<i32, i32>,
}

impl InjuryManager {
    pub(super) fn begin_round(&mut self) {
        self.round_counts.clear();
    }

    pub fn round_count(&self, team_type: i32) -> i32 {
        self.round_counts
            .get(&team_type)
            .copied()
            .unwrap_or_default()
    }

    pub fn execute(&mut self, command: InjuryCommand) -> InjuryChange {
        match command {
            InjuryCommand::RecordAction(record) => {
                let mut targets = record.injured_targets;
                targets.sort_unstable();
                targets.dedup();
                let before = self.round_count(record.team_type);
                let after = before.saturating_add(targets.len() as i32);
                self.round_counts.insert(record.team_type, after);
                InjuryChange {
                    origin: record.origin,
                    source_uid: record.source_uid,
                    team_type: record.team_type,
                    counter_owner_uid: record.counter_owner_uid,
                    before,
                    after,
                }
            }
            InjuryCommand::Reset { origin, team_type } => {
                let before = self.round_counts.remove(&team_type).unwrap_or_default();
                InjuryChange {
                    origin,
                    source_uid: 0,
                    team_type,
                    counter_owner_uid: 0,
                    before,
                    after: 0,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::skill::rule::{DefinitionKey, RuleDomain};

    fn record(targets: &[i64]) -> InjuryCommand {
        InjuryCommand::RecordAction(InjuryRecord {
            origin: CommandOrigin {
                domain: RuleDomain::Skill,
                key: DefinitionKey::new(1, "TeamInjury"),
            },
            source_uid: 10,
            team_type: 1,
            injured_targets: targets.to_vec(),
            counter_owner_uid: 11,
        })
    }

    #[test]
    fn records_distinct_injured_allies_and_resets_each_round() {
        let mut manager = InjuryManager::default();
        let first = manager.execute(record(&[10, 10, 11]));
        let second = manager.execute(record(&[12]));

        assert_eq!((first.before, first.after), (0, 2));
        assert_eq!((second.before, second.after), (2, 3));
        manager.begin_round();
        assert_eq!(manager.round_count(1), 0);
    }

    #[test]
    fn reset_command_starts_a_new_visible_counter_cycle() {
        let mut manager = InjuryManager::default();
        manager.execute(record(&[10, 11]));
        let origin = CommandOrigin {
            domain: RuleDomain::Condition,
            key: DefinitionKey::new(578, "TeamInjuryCountRound"),
        };

        let reset = manager.execute(InjuryCommand::Reset {
            origin,
            team_type: 1,
        });
        let next = manager.execute(record(&[12]));

        assert_eq!((reset.before, reset.after), (2, 0));
        assert_eq!((next.before, next.after), (0, 1));
    }
}
