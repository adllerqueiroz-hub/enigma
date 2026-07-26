use super::*;

#[tokio::test]
async fn handbook_read_persists_known_type() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at) VALUES (11, 'book', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();

    handbook_read(&pool, 11, 3, 3125).await.unwrap();
    assert!(handbook_read(&pool, 11, 5, 3125).await.is_err());
    let is_read: bool = sqlx::query_scalar(
        "SELECT is_read FROM user_handbook_reads
             WHERE user_id = 11 AND type = 3 AND handbook_id = 3125",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(is_read);
}

#[tokio::test]
async fn power_maker_only_reports_offline_progress_during_login() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    database::run_migrations(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, created_at, updated_at) VALUES (12, 'power', 0, 0)",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO user_power_maker_state
             (user_id, status, next_remain_second, make_count, logout_second)
             VALUES (12, 1, 36123, 28, 5866693)",
    )
    .execute(&pool)
    .await
    .unwrap();

    let login = power_maker_info(&pool, 12, true).await.unwrap();
    let refresh = power_maker_info(&pool, 12, false).await.unwrap();
    assert_eq!(
        (login.make_count, login.logout_second),
        (Some(28), Some(5866693))
    );
    assert_eq!(
        (refresh.make_count, refresh.logout_second),
        (Some(0), Some(0))
    );
}
