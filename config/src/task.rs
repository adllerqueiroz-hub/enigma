use crate::{
    GameDB, task_daily::TaskDaily, task_guide::TaskGuide, task_room::TaskRoom,
    task_season::TaskSeason, task_weekly::TaskWeekly,
};

impl GameDB {
    pub fn online_daily_tasks(&self) -> impl Iterator<Item = &TaskDaily> {
        self.task_daily.iter().filter(|row| row.is_online != 0)
    }

    pub fn online_weekly_tasks(&self) -> impl Iterator<Item = &TaskWeekly> {
        self.task_weekly.iter().filter(|row| row.is_online != 0)
    }

    pub fn online_guide_tasks(&self) -> impl Iterator<Item = &TaskGuide> {
        self.task_guide.iter().filter(|row| row.is_online != 0)
    }

    pub fn online_room_tasks(&self) -> impl Iterator<Item = &TaskRoom> {
        self.task_room.iter().filter(|row| row.is_online != 0)
    }

    pub fn online_season_tasks(&self) -> impl Iterator<Item = &TaskSeason> {
        self.task_season.iter().filter(|row| row.is_online != 0)
    }
}
