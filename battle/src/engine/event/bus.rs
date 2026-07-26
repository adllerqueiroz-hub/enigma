use std::collections::VecDeque;

use crate::engine::event::{kind::EventKind, payload::BattleEvent};

#[derive(Debug, Clone, Default)]
pub struct EventBus {
    events: VecDeque<BattleEvent>,
}

impl EventBus {
    pub fn push(&mut self, event: BattleEvent) {
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<BattleEvent> {
        self.events.pop_front()
    }

    pub fn drain_kind(&mut self, kind: EventKind) -> Vec<BattleEvent> {
        let mut matched = Vec::new();
        let mut kept = VecDeque::new();

        while let Some(event) = self.pop() {
            if event.kind() == kind {
                matched.push(event);
            } else {
                kept.push_back(event);
            }
        }

        self.events = kept;
        matched
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pops_events_in_published_order() {
        let mut bus = EventBus::default();
        bus.push(BattleEvent::EnterFight);
        bus.push(BattleEvent::RoundStart);

        assert_eq!(bus.pop(), Some(BattleEvent::EnterFight));
        assert_eq!(bus.pop(), Some(BattleEvent::RoundStart));
        assert!(bus.pop().is_none());
    }

    #[test]
    fn draining_one_kind_preserves_both_orders() {
        let mut bus = EventBus::default();
        bus.push(BattleEvent::EnterFight);
        bus.push(BattleEvent::RoundStart);
        bus.push(BattleEvent::Kind(EventKind::RoundStart));
        bus.push(BattleEvent::Kind(EventKind::SkillAction));

        assert_eq!(
            bus.drain_kind(EventKind::RoundStart),
            vec![
                BattleEvent::RoundStart,
                BattleEvent::Kind(EventKind::RoundStart)
            ]
        );
        assert_eq!(bus.pop(), Some(BattleEvent::EnterFight));
        assert_eq!(bus.pop(), Some(BattleEvent::Kind(EventKind::SkillAction)));
    }
}
