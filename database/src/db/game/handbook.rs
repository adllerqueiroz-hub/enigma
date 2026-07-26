use anyhow::Result;
use sonettobuf::{ChatperElementInfo, Handbook};
use sqlx::SqlitePool;

pub async fn get_handbook_reads(pool: &SqlitePool, user_id: i64) -> Result<Vec<Handbook>> {
    let rows = sqlx::query_as::<_, (i32, i32, bool)>(
        r#"
        SELECT type, handbook_id, is_read
        FROM user_handbook_reads
        WHERE user_id = ?
        ORDER BY type, handbook_id
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(r#type, id, is_read)| Handbook {
            r#type: Some(r#type),
            id: Some(id),
            is_read: Some(is_read),
        })
        .collect())
}

pub async fn get_handbook_fragments(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<ChatperElementInfo>> {
    let rows = sqlx::query_as::<_, (i32, String)>(
        r#"
        SELECT element, dialog_ids
        FROM user_handbook_fragments
        WHERE user_id = ?
        ORDER BY element
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|(element, dialog_ids)| {
            Ok(ChatperElementInfo {
                element: Some(element),
                dialog_ids: serde_json::from_str(&dialog_ids)?,
            })
        })
        .collect()
}

pub async fn mark_read(
    pool: &SqlitePool,
    user_id: i64,
    r#type: i32,
    handbook_id: i32,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO user_handbook_reads (user_id, type, handbook_id, is_read)
         VALUES (?, ?, ?, 1)
         ON CONFLICT(user_id, type, handbook_id) DO UPDATE SET is_read = 1",
    )
    .bind(user_id)
    .bind(r#type)
    .bind(handbook_id)
    .execute(pool)
    .await?;
    Ok(())
}
