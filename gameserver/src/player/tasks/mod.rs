use crate::{
    error::AppError,
    logic::{reward, room},
};
use database::db::game::tasks as task_db;
use sonettobuf::{
    FinishAllTaskReply, FinishReadTaskReply, FinishTaskReply, GetTaskActivityBonusReply,
    GetTaskInfoReply, RefreshOnlineTaskReply, Task, TaskActivityInfo,
};
use sqlx::SqlitePool;
use std::collections::HashMap;

mod rewards;

use rewards::{
    add_claim_activity_in_transaction, parse_task_reward, task_activity_bonus, task_rewards,
};

#[derive(Clone, Debug)]
pub struct TaskManager {
    player_id: i64,
    tasks: HashMap<(i32, i32), database::models::game::tasks::UserTask>,
}

impl TaskManager {
    pub fn new(player_id: i64) -> Self {
        Self {
            player_id,
            tasks: HashMap::new(),
        }
    }

    pub async fn get_info(
        &mut self,
        db: &SqlitePool,
        type_ids: Vec<u32>,
    ) -> Result<GetTaskInfoReply, AppError> {
        let db_type_ids = type_ids
            .iter()
            .map(|type_id| *type_id as i32)
            .collect::<Vec<_>>();
        if db_type_ids.is_empty() || db_type_ids.contains(&task_db::TaskType::Room.id()) {
            room::sync_room_tasks(db, config::configs::get(), self.player_id).await?;
        }
        let tasks = task_db::list_by_types(db, self.player_id, db_type_ids.clone()).await?;
        self.cache_tasks(&tasks);
        let activity = task_db::list_activity(db, self.player_id, db_type_ids)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(GetTaskInfoReply {
            task_info: tasks.into_iter().map(Into::into).collect(),
            activity_info: activity,
            type_ids,
        })
    }

    pub async fn finish(
        &mut self,
        db: &SqlitePool,
        task_id: i32,
    ) -> Result<TaskClaim<FinishTaskReply>, AppError> {
        if config::configs::get().task_room.get(task_id).is_some() {
            room::sync_room_tasks(db, config::configs::get(), self.player_id).await?;
        }
        let task = task_db::get_by_id(db, self.player_id, task_id)
            .await?
            .ok_or(AppError::InvalidRequest)?;
        let mut tx = db.begin().await?;
        let task = task_db::finish_task_in_transaction(&mut tx, &task)
            .await?
            .ok_or(AppError::InvalidRequest)?;
        let activity = add_claim_activity_in_transaction(&mut tx, self.player_id, &task).await?;
        let reward_set = task_rewards(task.type_id, task.task_id);
        let material_changes = reward_set.material_changes();
        let rewards = reward::apply_in_transaction(&mut tx, db, self.player_id, reward_set).await?;
        tx.commit().await?;
        self.cache_task(task.clone());

        Ok(TaskClaim {
            reply: FinishTaskReply {
                id: Some(task.task_id),
                finish_count: Some(task.finish_count),
            },
            task_info: vec![task.into()],
            activity_info: activity.into_iter().map(Into::into).collect(),
            rewards,
            material_changes,
        })
    }

    pub async fn finish_all(
        &mut self,
        db: &SqlitePool,
        type_id: i32,
        min_type_id: Option<i32>,
        task_ids: Vec<i32>,
        activity_id: Option<i32>,
    ) -> Result<TaskClaim<FinishAllTaskReply>, AppError> {
        if type_id == task_db::TaskType::Room.id() {
            room::sync_room_tasks(db, config::configs::get(), self.player_id).await?;
        }
        let claimable = task_db::claimable_tasks(
            db,
            self.player_id,
            type_id,
            min_type_id,
            activity_id,
            task_ids.clone(),
        )
        .await?;
        let mut tx = db.begin().await?;
        let tasks = task_db::finish_tasks_in_transaction(&mut tx, &claimable)
            .await?
            .ok_or(AppError::InvalidRequest)?;

        let mut activity = Vec::new();
        let mut reward_set = reward::RewardSet::default();
        for task in &tasks {
            activity
                .extend(add_claim_activity_in_transaction(&mut tx, self.player_id, task).await?);
            reward_set.extend(task_rewards(task.type_id, task.task_id));
        }

        let material_changes = reward_set.material_changes();
        let rewards = reward::apply_in_transaction(&mut tx, db, self.player_id, reward_set).await?;
        tx.commit().await?;
        self.cache_tasks(&tasks);

        Ok(TaskClaim {
            reply: FinishAllTaskReply {
                type_id: Some(type_id),
                min_type_id: Some(min_type_id.unwrap_or_default()),
                task_ids,
                activity_id,
            },
            task_info: tasks.into_iter().map(Into::into).collect(),
            activity_info: activity.into_iter().map(Into::into).collect(),
            rewards,
            material_changes,
        })
    }

    pub async fn get_activity_bonus(
        &mut self,
        db: &SqlitePool,
        type_id: i32,
        define_id: i32,
    ) -> Result<TaskClaim<GetTaskActivityBonusReply>, AppError> {
        let mut activity_info = Vec::new();
        let mut reward_set = reward::RewardSet::default();

        if let Some(bonus) = task_activity_bonus(type_id, define_id) {
            let mut tx = db.begin().await?;
            let (activity, claimed) = task_db::claim_activity_bonus_in_transaction(
                &mut tx,
                self.player_id,
                type_id,
                define_id,
                bonus.need_activity,
            )
            .await?;
            activity_info.push(activity.into());

            if claimed {
                reward_set = parse_task_reward(&bonus.bonus);
            }
            let material_changes = reward_set.material_changes();
            let rewards =
                reward::apply_in_transaction(&mut tx, db, self.player_id, reward_set).await?;
            tx.commit().await?;

            return Ok(TaskClaim {
                reply: GetTaskActivityBonusReply {
                    type_id: Some(type_id),
                    define_id: Some(define_id),
                },
                task_info: Vec::new(),
                activity_info,
                rewards,
                material_changes,
            });
        }

        let material_changes = reward_set.material_changes();
        let rewards = reward::apply(db, self.player_id, reward_set).await?;

        Ok(TaskClaim {
            reply: GetTaskActivityBonusReply {
                type_id: Some(type_id),
                define_id: Some(define_id),
            },
            task_info: Vec::new(),
            activity_info,
            rewards,
            material_changes,
        })
    }

    pub async fn finish_read(
        &mut self,
        db: &SqlitePool,
        task_id: Option<i32>,
    ) -> Result<(FinishReadTaskReply, Option<Task>), AppError> {
        let task_id = task_id.ok_or(AppError::InvalidRequest)?;
        let task = task_db::read_task(db, self.player_id, task_id).await?;
        if let Some(task) = &task {
            self.cache_task(task.clone());
        }

        Ok((
            FinishReadTaskReply {
                task_id: Some(task_id),
            },
            task.map(Into::into),
        ))
    }

    fn cache_tasks(&mut self, tasks: &[database::models::game::tasks::UserTask]) {
        for task in tasks {
            self.cache_task(task.clone());
        }
    }

    fn cache_task(&mut self, task: database::models::game::tasks::UserTask) {
        self.tasks.insert((task.type_id, task.task_id), task);
    }
}

pub struct TaskClaim<T> {
    pub reply: T,
    pub task_info: Vec<Task>,
    pub activity_info: Vec<TaskActivityInfo>,
    pub rewards: reward::AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub fn refresh_online_task(id: Option<i32>) -> RefreshOnlineTaskReply {
    RefreshOnlineTaskReply { id }
}
