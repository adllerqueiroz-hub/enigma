use anyhow::Result;
use sqlx::SqlitePool;

pub async fn get_friend_ids(pool: &SqlitePool, user_id: i64) -> Result<Vec<u64>> {
    let friends = sqlx::query_scalar(
        "SELECT friend_id FROM user_friends WHERE user_id = ? ORDER BY created_at",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(friends.into_iter().map(|id: i64| id as u64).collect())
}

pub async fn get_blacklist_ids(pool: &SqlitePool, user_id: i64) -> Result<Vec<u64>> {
    let blacklist = sqlx::query_scalar(
        "SELECT blocked_user_id FROM user_blacklist WHERE user_id = ? ORDER BY created_at",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(blacklist.into_iter().map(|id: i64| id as u64).collect())
}

pub async fn get_recommended_ids(pool: &SqlitePool, user_id: i64, limit: i64) -> Result<Vec<u64>> {
    let recommended = sqlx::query_scalar(
        r#"
        SELECT id
        FROM users
        WHERE id != ?
          AND NOT EXISTS (
              SELECT 1 FROM user_friends
              WHERE user_friends.user_id = ? AND user_friends.friend_id = users.id
          )
          AND NOT EXISTS (
              SELECT 1 FROM user_blacklist
              WHERE user_blacklist.user_id = ? AND user_blacklist.blocked_user_id = users.id
          )
        ORDER BY last_login_at DESC, id
        LIMIT ?
        "#,
    )
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(recommended.into_iter().map(|id: i64| id as u64).collect())
}

pub async fn search_user_ids(pool: &SqlitePool, user_id: i64, value: String) -> Result<Vec<u64>> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Ok(Vec::new());
    }

    let numeric_id = value.parse::<i64>().ok();
    let ids = sqlx::query_scalar(
        r#"
        SELECT id
        FROM users
        WHERE id != ?
          AND (username = ? OR id = ?)
        ORDER BY id
        LIMIT 20
        "#,
    )
    .bind(user_id)
    .bind(value)
    .bind(numeric_id)
    .fetch_all(pool)
    .await?;

    Ok(ids.into_iter().map(|id: i64| id as u64).collect())
}

pub async fn add_friend(pool: &SqlitePool, user_id: i64, friend_id: i64) -> Result<()> {
    let now = chrono::Utc::now().timestamp();

    sqlx::query(
        "INSERT INTO user_friends (user_id, friend_id, created_at) VALUES (?, ?, ?) ON CONFLICT DO NOTHING"
    )
    .bind(user_id)
    .bind(friend_id)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_friend(pool: &SqlitePool, user_id: i64, friend_id: i64) -> Result<()> {
    sqlx::query("DELETE FROM user_friends WHERE user_id = ? AND friend_id = ?")
        .bind(user_id)
        .bind(friend_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn add_to_blacklist(pool: &SqlitePool, user_id: i64, blocked_id: i64) -> Result<()> {
    let now = chrono::Utc::now().timestamp();

    sqlx::query(
        "INSERT INTO user_blacklist (user_id, blocked_user_id, created_at) VALUES (?, ?, ?) ON CONFLICT DO NOTHING"
    )
    .bind(user_id)
    .bind(blocked_id)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove_from_blacklist(pool: &SqlitePool, user_id: i64, blocked_id: i64) -> Result<()> {
    sqlx::query("DELETE FROM user_blacklist WHERE user_id = ? AND blocked_user_id = ?")
        .bind(user_id)
        .bind(blocked_id)
        .execute(pool)
        .await?;

    Ok(())
}
