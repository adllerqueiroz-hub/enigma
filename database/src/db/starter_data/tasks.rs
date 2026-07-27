use crate::db::game::tasks::{self, TaskType};
use common::time::ServerTime;
use sqlx::{Sqlite, Transaction};

pub async fn load_starter_tasks(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
) -> sqlx::Result<()> {
    tasks::seed_configured_tasks_in_transaction(tx, user_id).await?;

    let now = ServerTime::now_ms();
    let daily_expiry = ServerTime::next_daily_refresh_sec(now);
    let weekly_expiry = ServerTime::next_weekly_refresh_sec(now);
    for task_type in TaskType::all() {
        let expiry_time = match task_type {
            TaskType::Daily => daily_expiry,
            TaskType::Weekly | TaskType::WeekWalk => weekly_expiry,
            _ => 0,
        };
        sqlx::query(
            "INSERT INTO user_task_activity
                (user_id, type_id, expiry_time, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(user_id)
        .bind(task_type.id())
        .bind(expiry_time)
        .bind(now)
        .bind(now)
        .execute(&mut **tx)
        .await?;
    }

    if let Some(bp_id) = tasks::current_battle_pass_id() {
        sqlx::query("INSERT INTO user_battle_pass_state (user_id, bp_id) VALUES (?, ?)")
            .bind(user_id)
            .bind(bp_id)
            .execute(&mut **tx)
            .await?;
    }

    Ok(())
}
