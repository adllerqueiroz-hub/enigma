use crate::{error::AppError, logic::reward};
use database::db::game::{handbook, power_maker, settings, unlock_voucher};
use sonettobuf::{
    GetHandbookInfoReply, GetPowerMakerInfoReply, GetSettingInfosReply, GetUnlockVoucherInfoReply,
    HandbookReadReply, UpdateSettingInfoReply,
};
use sqlx::SqlitePool;

pub struct RewardedReply<T> {
    pub reply: T,
    pub rewards: reward::AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn handbook_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetHandbookInfoReply, AppError> {
    Ok(GetHandbookInfoReply {
        infos: handbook::get_handbook_reads(db, player_id).await?,
        element_info: handbook::get_handbook_fragments(db, player_id).await?,
    })
}

pub async fn handbook_read(
    db: &SqlitePool,
    player_id: i64,
    r#type: i32,
    id: i32,
) -> Result<HandbookReadReply, AppError> {
    if !(1..=4).contains(&r#type) || id <= 0 {
        return Err(AppError::InvalidRequest);
    }
    handbook::mark_read(db, player_id, r#type, id).await?;
    Ok(HandbookReadReply {
        r#type: Some(r#type),
        id: Some(id),
    })
}

pub async fn setting_infos(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetSettingInfosReply, AppError> {
    Ok(GetSettingInfosReply {
        infos: settings::get_setting_infos(db, player_id).await?,
    })
}

pub async fn update_setting_info(
    db: &SqlitePool,
    player_id: i64,
    r#type: i32,
    param: String,
) -> Result<UpdateSettingInfoReply, AppError> {
    Ok(settings::update_setting_info(db, player_id, r#type, param).await?)
}

pub async fn unlock_voucher_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetUnlockVoucherInfoReply, AppError> {
    Ok(GetUnlockVoucherInfoReply {
        vouchers: unlock_voucher::get_unlock_vouchers(db, player_id).await?,
    })
}

pub async fn power_maker_info(
    db: &SqlitePool,
    player_id: i64,
    is_login: bool,
) -> Result<GetPowerMakerInfoReply, AppError> {
    let state = power_maker::get_state(db, player_id).await?;
    Ok(GetPowerMakerInfoReply {
        status: Some(state.status),
        next_remain_second: Some(state.next_remain_second),
        make_count: Some(if is_login { state.make_count } else { 0 }),
        logout_second: Some(if is_login { state.logout_second } else { 0 }),
        power_maker_items: power_maker::get_maker_items(db, player_id).await?,
    })
}

#[cfg(test)]
mod test;
