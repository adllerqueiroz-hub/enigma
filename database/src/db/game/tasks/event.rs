use crate::models::game::tasks::UserTask;
use sqlx::SqlitePool;

use crate::db::game::battle_pass;

use super::{TaskType, add_progress, current_battle_pass_id, ensure_tasks_for_type};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskEvent {
    Act205FinishGame {
        activity_id: i32,
        game_type: i32,
        is_win: bool,
    },
    CurrencyDec {
        currency_id: i32,
        amount: i32,
    },
    DoneCount {
        name: &'static str,
        count: i32,
    },
    DungeonPass {
        chapter_type: i32,
        count: i32,
    },
    EpisodeFinish {
        episode_id: i32,
    },
    ProductionLine {
        action: ProductionLineAction,
        count: i32,
    },
    StoreGoodsBought {
        store_id: i32,
    },
    TaskFinish {
        task_id: i32,
    },
    TowerMopUp {
        count: i32,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionLineAction {
    Gather = 1,
    Create = 2,
}

impl TaskEvent {
    pub fn hero_touch_count(self) -> Option<i32> {
        match self {
            Self::DoneCount {
                name: "HeroTouch",
                count,
            } => Some(count),
            _ => None,
        }
    }

    fn count(self) -> i32 {
        match self {
            Self::Act205FinishGame { .. } => 1,
            Self::CurrencyDec { amount, .. } => amount.max(1),
            Self::DoneCount { count, .. }
            | Self::DungeonPass { count, .. }
            | Self::ProductionLine { count, .. }
            | Self::TowerMopUp { count } => count.max(1),
            Self::EpisodeFinish { .. }
            | Self::StoreGoodsBought { .. }
            | Self::TaskFinish { .. } => 1,
        }
    }

    fn matches(self, listener_type: &str, listener_param: &str) -> bool {
        match self {
            Self::Act205FinishGame {
                activity_id,
                game_type,
                is_win,
            } => {
                (listener_type == "Act205FinishGame"
                    || (is_win && listener_type == "Act205GameWin"))
                    && matches_pair_param(listener_param, activity_id, game_type)
            }
            Self::CurrencyDec { currency_id, .. } => {
                listener_type == "CurrencyDec"
                    && listener_param
                        .parse::<i32>()
                        .is_ok_and(|param| param == currency_id)
            }
            Self::DoneCount { name, .. } => listener_type == "DoneCount" && listener_param == name,
            Self::DungeonPass { chapter_type, .. } => {
                listener_type == "DungeonPass" && contains_i32_param(listener_param, chapter_type)
            }
            Self::EpisodeFinish { episode_id } => {
                listener_type == "EpisodeFinish"
                    && listener_param
                        .parse::<i32>()
                        .is_ok_and(|param| param == episode_id)
            }
            Self::ProductionLine { action, .. } => {
                listener_type == "ProductionLine"
                    && listener_param
                        .parse::<i32>()
                        .is_ok_and(|param| param == action as i32)
            }
            Self::StoreGoodsBought { store_id } => {
                listener_type == "BuyStoreGoods" && listener_param.parse::<i32>() == Ok(store_id)
            }
            Self::TaskFinish { task_id } => {
                listener_type == "TaskFinish"
                    && listener_param
                        .strip_prefix("taskIds=")
                        .is_some_and(|ids| contains_i32_param(ids, task_id))
            }
            Self::TowerMopUp { .. } => listener_type == "TowerMopUpTime",
        }
    }

    pub(crate) fn achievement_increment(
        self,
        listener_type: &str,
        listener_param: &str,
    ) -> Option<i32> {
        match self {
            Self::CurrencyDec {
                currency_id,
                amount,
            } if listener_type == "CurrencyDecTotal"
                && listener_param.parse::<i32>() == Ok(currency_id) =>
            {
                Some(amount.max(1))
            }
            Self::DoneCount { name, count }
                if listener_type == "DoneCountTotal" && listener_param == name =>
            {
                Some(count.max(1))
            }
            _ => None,
        }
    }
}

pub async fn sync_event_tasks(
    pool: &SqlitePool,
    user_id: i64,
    event: TaskEvent,
) -> sqlx::Result<Vec<UserTask>> {
    ensure_tasks_for_type(pool, user_id, TaskType::Daily).await?;
    ensure_tasks_for_type(pool, user_id, TaskType::Weekly).await?;
    ensure_tasks_for_type(pool, user_id, TaskType::BattlePass).await?;
    ensure_tasks_for_type(pool, user_id, TaskType::BpOperAct).await?;
    ensure_tasks_for_type(pool, user_id, TaskType::Activity125).await?;

    let bp_id = current_battle_pass_id();
    let include_bp = match bp_id {
        Some(bp_id) => !battle_pass::score_maxed(pool, user_id, bp_id).await?,
        None => false,
    };
    let mut updated = Vec::new();
    for target in event_task_targets(event, bp_id.filter(|_| include_bp)) {
        if let Some(task) = add_progress(
            pool,
            user_id,
            target.type_id,
            target.task_id,
            event.count(),
            target.max_progress,
        )
        .await?
        {
            updated.push(task);
        }
    }

    Ok(updated)
}

fn event_task_targets(event: TaskEvent, bp_id: Option<i32>) -> Vec<TaskTarget> {
    let tables = config::configs::get();
    let mut targets = Vec::new();

    targets.extend(
        tables
            .online_daily_tasks()
            .filter(|task| event.matches(&task.listener_type, &task.listener_param))
            .map(|task| TaskTarget::new(TaskType::Daily, task.id, task.max_progress)),
    );
    targets.extend(
        tables
            .online_weekly_tasks()
            .filter(|task| event.matches(&task.listener_type, &task.listener_param))
            .map(|task| TaskTarget::new(TaskType::Weekly, task.id, task.max_progress)),
    );

    if let Some(bp_id) = bp_id {
        targets.extend(
            tables
                .battle_pass_tasks(bp_id)
                .filter(|task| {
                    task.is_online != 0 && event.matches(&task.listener_type, &task.listener_param)
                })
                .map(|task| TaskTarget::new(TaskType::BattlePass, task.id, task.max_progress)),
        );
        targets.extend(
            tables
                .activity214_task
                .iter()
                .filter(|task| {
                    task.bp_id == bp_id
                        && task.is_online != 0
                        && event.matches(&task.listener_type, &task.listener_param)
                })
                .map(|task| TaskTarget::new(TaskType::BpOperAct, task.id, task.max_progress)),
        );
    }
    targets.extend(
        tables
            .turnback_task
            .iter()
            .filter(|task| {
                task.is_online != 0 && event.matches(&task.listener_type, &task.listener_param)
            })
            .map(|task| TaskTarget::new(TaskType::Turnback, task.id, task.max_progress)),
    );
    targets.extend(
        tables
            .activity125_task
            .iter()
            .filter(|task| {
                task.is_online != 0 && event.matches(&task.listener_type, &task.listener_param)
            })
            .map(|task| TaskTarget::new(TaskType::Activity125, task.id, task.max_progress)),
    );

    targets
}

fn contains_i32_param(value: &str, expected: i32) -> bool {
    value
        .split('#')
        .filter_map(|part| part.parse::<i32>().ok())
        .any(|param| param == expected)
}

fn matches_pair_param(value: &str, first: i32, second: i32) -> bool {
    let mut parts = value.split('#').filter_map(|part| part.parse::<i32>().ok());
    parts.next() == Some(first) && parts.next() == Some(second)
}

struct TaskTarget {
    type_id: i32,
    task_id: i32,
    max_progress: i32,
}

impl TaskTarget {
    fn new(task_type: TaskType, task_id: i32, max_progress: i32) -> Self {
        Self {
            type_id: task_type.id(),
            task_id,
            max_progress,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tower_mop_up_routes_to_current_pass_and_returner_tasks() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);
        let targets =
            event_task_targets(TaskEvent::TowerMopUp { count: 4 }, current_battle_pass_id());

        assert!(
            targets
                .iter()
                .any(|target| target.type_id == TaskType::BpOperAct.id())
        );
        assert!(
            targets
                .iter()
                .any(|target| target.type_id == TaskType::Turnback.id())
        );
    }

    #[test]
    fn production_line_actions_keep_gather_and_create_tasks_distinct() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);

        let gather = event_task_targets(
            TaskEvent::ProductionLine {
                action: ProductionLineAction::Gather,
                count: 3,
            },
            None,
        );
        let create = event_task_targets(
            TaskEvent::ProductionLine {
                action: ProductionLineAction::Create,
                count: 1,
            },
            None,
        );

        assert!(gather.iter().any(|target| target.task_id == 10061));
        assert!(!gather.iter().any(|target| target.task_id == 20101));
        assert!(create.iter().any(|target| target.task_id == 20101));
        assert!(!create.iter().any(|target| target.task_id == 10061));
    }

    #[test]
    fn store_purchase_routes_by_store_id() {
        let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
        let _ = config::init(&data_dir);

        let targets = event_task_targets(TaskEvent::StoreGoodsBought { store_id: 111 }, None);
        assert!(targets.iter().any(|target| target.task_id == 180054));
        assert!(
            event_task_targets(TaskEvent::StoreGoodsBought { store_id: 614 }, None)
                .iter()
                .all(|target| target.task_id != 180054)
        );
    }
}
