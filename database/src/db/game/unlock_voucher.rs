use anyhow::Result;
use sonettobuf::UnlockVoucherInfo;
use sqlx::SqlitePool;

pub async fn get_unlock_vouchers(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<UnlockVoucherInfo>> {
    let rows = sqlx::query_as::<_, (i32, i64)>(
        r#"
        SELECT voucher_id, get_time
        FROM user_unlock_vouchers
        WHERE user_id = ?
        ORDER BY voucher_id
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(voucher_id, get_time)| UnlockVoucherInfo {
            voucher_id: Some(voucher_id),
            get_time: Some(get_time as u64),
        })
        .collect())
}
