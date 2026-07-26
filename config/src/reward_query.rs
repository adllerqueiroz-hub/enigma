use crate::{GameDB, reward::Reward, reward_group::RewardGroup};

impl GameDB {
    pub fn reward(&self, reward_id: i32) -> Option<&Reward> {
        self.reward
            .iter()
            .find(|reward| reward.reward_id == reward_id)
    }

    pub fn reward_group(&self, group: &str) -> impl Iterator<Item = &RewardGroup> {
        self.reward_group
            .iter()
            .filter(move |reward| reward.group == group)
    }
}
