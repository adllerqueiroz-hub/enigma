use sonettobuf::PlayerCloth;
use sqlx::{Sqlite, SqlitePool, Transaction};

pub async fn get_all(pool: &SqlitePool, user_id: i64) -> sqlx::Result<Vec<PlayerCloth>> {
    let rows: Vec<(i32, i32, i32)> = sqlx::query_as(
        "SELECT cloth_id, level, exp
         FROM user_cloths
         WHERE user_id = ?
         ORDER BY cloth_id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(cloth_id, level, exp)| PlayerCloth {
            cloth_id: Some(cloth_id),
            level: Some(level),
            exp: Some(exp),
        })
        .collect())
}

pub async fn unlock(pool: &SqlitePool, user_id: i64, cloth_id: i32) -> sqlx::Result<PlayerCloth> {
    let mut tx = pool.begin().await?;
    let cloth = unlock_in_transaction(&mut tx, user_id, cloth_id).await?;
    tx.commit().await?;
    Ok(cloth)
}

pub async fn unlock_in_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    cloth_id: i32,
) -> sqlx::Result<PlayerCloth> {
    sqlx::query(
        "INSERT INTO user_cloths (user_id, cloth_id, level, exp)
         VALUES (?, ?, 1, 0)
         ON CONFLICT(user_id, cloth_id) DO NOTHING",
    )
    .bind(user_id)
    .bind(cloth_id)
    .execute(&mut **tx)
    .await?;

    let (level, exp): (i32, i32) =
        sqlx::query_as("SELECT level, exp FROM user_cloths WHERE user_id = ? AND cloth_id = ?")
            .bind(user_id)
            .bind(cloth_id)
            .fetch_one(&mut **tx)
            .await?;

    Ok(PlayerCloth {
        cloth_id: Some(cloth_id),
        level: Some(level),
        exp: Some(exp),
    })
}

pub async fn add_exp(
    pool: &SqlitePool,
    user_id: i64,
    cloth_id: i32,
    amount: i32,
) -> sqlx::Result<PlayerCloth> {
    let mut tx = pool.begin().await?;
    let cloth = add_exp_in_transaction(&mut tx, user_id, cloth_id, amount).await?;
    tx.commit().await?;
    Ok(cloth)
}

pub async fn add_exp_in_transaction(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    cloth_id: i32,
    amount: i32,
) -> sqlx::Result<PlayerCloth> {
    sqlx::query(
        "INSERT INTO user_cloths (user_id, cloth_id, level, exp)
         VALUES (?, ?, 1, ?)
         ON CONFLICT(user_id, cloth_id) DO UPDATE SET exp = exp + excluded.exp",
    )
    .bind(user_id)
    .bind(cloth_id)
    .bind(amount)
    .execute(&mut **tx)
    .await?;

    let (level, exp): (i32, i32) =
        sqlx::query_as("SELECT level, exp FROM user_cloths WHERE user_id = ? AND cloth_id = ?")
            .bind(user_id)
            .bind(cloth_id)
            .fetch_one(&mut **tx)
            .await?;

    Ok(PlayerCloth {
        cloth_id: Some(cloth_id),
        level: Some(level),
        exp: Some(exp),
    })
}
