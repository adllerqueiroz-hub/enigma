use anyhow::Result;
use sonettobuf::{SettingInfo, UpdateSettingInfoReply};
use sqlx::SqlitePool;

#[derive(Clone, Copy)]
enum PushSettingType {
    Reactivation = 1,
    RoomProduceUpperLimit = 2,
    AllowRecommend = 3,
}

pub(crate) const DEFAULT_PUSH_SETTING_TYPES: [i32; 3] = [
    PushSettingType::Reactivation as i32,
    PushSettingType::RoomProduceUpperLimit as i32,
    PushSettingType::AllowRecommend as i32,
];

pub async fn get_setting_infos(pool: &SqlitePool, user_id: i64) -> Result<Vec<SettingInfo>> {
    let rows = sqlx::query_as::<_, (i32, String)>(
        "SELECT type, param FROM user_setting_infos WHERE user_id = ? ORDER BY type",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(r#type, param)| SettingInfo {
            r#type: Some(r#type),
            param: Some(param),
        })
        .collect())
}

pub async fn update_setting_info(
    pool: &SqlitePool,
    user_id: i64,
    r#type: i32,
    param: String,
) -> Result<UpdateSettingInfoReply> {
    sqlx::query(
        r#"
        INSERT INTO user_setting_infos (user_id, type, param)
        VALUES (?, ?, ?)
        ON CONFLICT(user_id, type) DO UPDATE SET param = excluded.param
        "#,
    )
    .bind(user_id)
    .bind(r#type)
    .bind(&param)
    .execute(pool)
    .await?;

    Ok(UpdateSettingInfoReply {
        r#type: Some(r#type),
        param: Some(param),
    })
}
