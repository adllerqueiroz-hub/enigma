use super::*;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::test]
async fn claimed_task_is_persisted_as_no_longer_claimable() {
    let data_dir = format!("{}/../data/excel2json", env!("CARGO_MANIFEST_DIR"));
    let _ = config::init(&data_dir);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at) VALUES (1, 'task', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO user_tasks
         (user_id, type_id, task_id, progress, has_finished, finish_count)
         VALUES (1, 1, 40100, 1, 1, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();

    let task = task_db::get_by_id(&pool, 1, 40100).await.unwrap().unwrap();
    let mut tx = pool.begin().await.unwrap();
    let claimed = task_db::finish_task_in_transaction(&mut tx, &task)
        .await
        .unwrap()
        .unwrap();
    tx.commit().await.unwrap();

    assert_eq!(claimed.finish_count, 1);
    assert!(!claimed.has_finished);
    let stored = task_db::get_by_id(&pool, 1, 40100).await.unwrap().unwrap();
    assert_eq!(stored.finish_count, 1);
    assert!(!stored.has_finished);

    task_db::sync_progress(&pool, 1, 1, 40100, 1, 1)
        .await
        .unwrap();
    let synced = task_db::get_by_id(&pool, 1, 40100).await.unwrap().unwrap();
    assert!(!synced.has_finished);
}
