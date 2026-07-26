use crate::{
    error::AppError,
    reward::{self, AppliedRewards},
    store,
};
use database::db::game::{charges, sign_in};
use sonettobuf::{GetChargeInfoReply, GetMonthCardBonusReply, GetMonthCardInfoReply};
use sqlx::SqlitePool;

pub struct MonthCardClaim {
    pub reply: GetMonthCardBonusReply,
    pub rewards: Option<AppliedRewards>,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn charge_info(db: &SqlitePool, player_id: i64) -> Result<GetChargeInfoReply, AppError> {
    let settings = charges::get_sandbox_settings(db, player_id).await?;

    Ok(GetChargeInfoReply {
        infos: store::charge_infos(db, player_id).await?,
        sandbox_enable: settings.sandbox_enable.then_some(true),
        sandbox_balance: settings.sandbox_enable.then_some(settings.sandbox_balance),
    })
}

pub async fn month_card_info(
    db: &SqlitePool,
    player_id: i64,
) -> Result<GetMonthCardInfoReply, AppError> {
    Ok(GetMonthCardInfoReply {
        infos: sign_in::get_month_card_infos(db, player_id).await?,
    })
}

pub async fn month_card_bonus(
    db: &SqlitePool,
    player_id: i64,
    id: Option<i32>,
) -> Result<MonthCardClaim, AppError> {
    let id = match id {
        Some(id) => id,
        None => month_card_info(db, player_id)
            .await?
            .infos
            .first()
            .and_then(|info| info.id)
            .unwrap_or_default(),
    };

    let mut tx = db.begin().await?;
    let mut rewards = None;
    let mut material_changes = Vec::new();
    if sign_in::claim_month_card_bonus_in_transaction(&mut tx, player_id, id).await?
        && let Some(row) = config::configs::get().month_card.get(id)
    {
        let parsed = reward::parse(&row.daily_bonus);
        material_changes = parsed.material_changes();
        rewards = Some(reward::apply_in_transaction(&mut tx, db, player_id, parsed).await?);
    }
    tx.commit().await?;

    Ok(MonthCardClaim {
        reply: GetMonthCardBonusReply { id: Some(id) },
        rewards,
        material_changes,
    })
}
