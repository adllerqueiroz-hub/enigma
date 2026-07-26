use crate::{
    GameDB, activity104_episode::Activity104Episode, activity104_retail::Activity104Retail,
    activity104_special::Activity104Special, activity104_trial::Activity104Trial,
    activity165_step::Activity165Step,
};

impl GameDB {
    pub fn latest_open_activity_id(&self, type_id: i32) -> Option<i32> {
        self.activity
            .iter()
            .filter(|activity| {
                activity.type_id == type_id
                    && (activity.open_id == 0
                        || self
                            .open
                            .get(activity.open_id)
                            .is_some_and(|open| open.is_online != 0))
            })
            .map(|activity| activity.id)
            .max()
    }

    pub fn latest_activity104_id(&self) -> Option<i32> {
        self.activity104_episode
            .iter()
            .map(|row| row.activity_id)
            .max()
    }

    pub fn activity104_episodes(
        &self,
        activity_id: i32,
    ) -> impl Iterator<Item = &Activity104Episode> {
        self.activity104_episode
            .iter()
            .filter(move |row| row.activity_id == activity_id)
    }

    pub fn activity104_episode(&self, activity_id: i32, layer: i32) -> Option<&Activity104Episode> {
        self.activity104_episodes(activity_id)
            .find(|row| row.layer == layer)
    }

    pub fn activity104_specials(
        &self,
        activity_id: i32,
    ) -> impl Iterator<Item = &Activity104Special> {
        self.activity104_special
            .iter()
            .filter(move |row| row.activity_id == activity_id)
    }

    pub fn activity104_retails(
        &self,
        activity_id: i32,
    ) -> impl Iterator<Item = &Activity104Retail> {
        self.activity104_retail
            .iter()
            .filter(move |row| row.activity_id == activity_id)
    }

    pub fn activity104_trial(&self, activity_id: i32) -> Option<&Activity104Trial> {
        self.activity104_trial
            .iter()
            .find(|row| row.activity_id == activity_id)
    }

    pub fn activity165_step(&self, story_id: i32, step_id: i32) -> Option<&Activity165Step> {
        self.activity165_step
            .iter()
            .find(|row| row.belong_story_id == story_id && row.step_id == step_id)
    }

    pub fn activity165_steps(&self, story_id: i32) -> impl Iterator<Item = &Activity165Step> {
        self.activity165_step
            .iter()
            .filter(move |row| row.belong_story_id == story_id)
    }
}
