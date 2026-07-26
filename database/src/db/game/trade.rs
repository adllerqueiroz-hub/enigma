use anyhow::Result;
use sonettobuf::TradeTaskInfo;
use sqlx::SqlitePool;

pub async fn get_trade_tasks(pool: &SqlitePool, user_id: i64) -> Result<Vec<TradeTaskInfo>> {
    let rows = sqlx::query_as::<_, (i32, i32, bool, bool, i32)>(
        "SELECT task_id, progress, has_finish, is_new, finish_time
         FROM user_trade_tasks
         WHERE user_id = ?
         ORDER BY task_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(
            |(id, progress, has_finish, is_new, finish_time)| TradeTaskInfo {
                id: Some(id),
                progress: Some(progress),
                has_finish: Some(has_finish),
                new: Some(is_new),
                finish_time: Some(finish_time),
            },
        )
        .collect())
}

pub async fn read_new_trade_tasks(pool: &SqlitePool, user_id: i64, ids: &[i32]) -> Result<()> {
    for id in ids {
        sqlx::query("UPDATE user_trade_tasks SET is_new = 0 WHERE user_id = ? AND task_id = ?")
            .bind(user_id)
            .bind(id)
            .execute(pool)
            .await?;
    }
    Ok(())
}

pub async fn get_support_bonus_ids(pool: &SqlitePool, user_id: i64) -> Result<Vec<i32>> {
    Ok(sqlx::query_scalar(
        "SELECT bonus_id FROM user_trade_support_bonus WHERE user_id = ? ORDER BY bonus_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?)
}

pub async fn claim_support_bonus(pool: &SqlitePool, user_id: i64, bonus_id: i32) -> Result<bool> {
    let result = sqlx::query(
        "INSERT OR IGNORE INTO user_trade_support_bonus (user_id, bonus_id) VALUES (?, ?)",
    )
    .bind(user_id)
    .bind(bonus_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn claim_support_bonus_in_transaction(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    user_id: i64,
    bonus_id: i32,
) -> Result<bool> {
    let result = sqlx::query(
        "INSERT OR IGNORE INTO user_trade_support_bonus (user_id, bonus_id) VALUES (?, ?)",
    )
    .bind(user_id)
    .bind(bonus_id)
    .execute(&mut **tx)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn finished_task_count(pool: &SqlitePool, user_id: i64) -> Result<i32> {
    Ok(sqlx::query_scalar(
        "SELECT COUNT(*) FROM user_trade_tasks WHERE user_id = ? AND has_finish = 1",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?)
}

pub async fn sync_tasks(
    pool: &SqlitePool,
    user_id: i64,
    trade_level: i32,
    tables: &config::GameDB,
) -> Result<()> {
    for row in tables
        .trade_task
        .iter()
        .filter(|task| task.trade_level <= trade_level)
    {
        sqlx::query(
            "INSERT OR IGNORE INTO user_trade_tasks (user_id, task_id)
             VALUES (?, ?)",
        )
        .bind(user_id)
        .bind(row.id)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn sync_tasks_in_transaction(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    user_id: i64,
    trade_level: i32,
    tables: &config::GameDB,
) -> Result<()> {
    for row in tables
        .trade_task
        .iter()
        .filter(|task| task.trade_level <= trade_level)
    {
        sqlx::query(
            "INSERT OR IGNORE INTO user_trade_tasks (user_id, task_id)
             VALUES (?, ?)",
        )
        .bind(user_id)
        .bind(row.id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}
