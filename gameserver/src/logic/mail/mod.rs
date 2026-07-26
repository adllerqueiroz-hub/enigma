use crate::{
    error::AppError,
    logic::reward::{self, AppliedRewards},
    player::red_dot,
};
use database::db::game::mail;
use sonettobuf::{
    DeleteMailBatchReply, GetAllMailsReply, MailLockReply, MarkMailJumpReply, ReadMailBatchReply,
    ReadMailReply,
};
use sqlx::SqlitePool;

#[derive(Default)]
pub struct MailClaimOutcome {
    pub incr_ids: Vec<i64>,
    pub rewards: AppliedRewards,
    pub material_changes: Vec<(u32, u32, i32)>,
    pub mail_red_dot: Option<(i32, i32)>,
}

pub async fn get_all(db: &SqlitePool, player_id: i64) -> Result<GetAllMailsReply, AppError> {
    Ok(GetAllMailsReply {
        mails: mail::get_all(db, player_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    })
}

pub async fn set_lock(
    db: &SqlitePool,
    player_id: i64,
    incr_id: i64,
    lock: bool,
) -> Result<MailLockReply, AppError> {
    if !mail::set_lock(db, player_id, incr_id, lock).await? {
        return Err(AppError::InvalidRequest);
    }

    Ok(MailLockReply {
        incr_id: Some(incr_id as u64),
        lock: Some(lock),
    })
}

pub async fn delete_claimed_unlocked(
    db: &SqlitePool,
    player_id: i64,
) -> Result<DeleteMailBatchReply, AppError> {
    Ok(DeleteMailBatchReply {
        incr_ids: mail::delete_claimed_unlocked(db, player_id)
            .await?
            .into_iter()
            .map(|id| id as u64)
            .collect(),
    })
}

pub async fn mark_jump(
    db: &SqlitePool,
    player_id: i64,
    incr_id: i64,
) -> Result<MarkMailJumpReply, AppError> {
    if !mail::mark_jump(db, player_id, incr_id).await? {
        return Err(AppError::InvalidRequest);
    }
    Ok(MarkMailJumpReply {
        incr_id: Some(incr_id as u64),
    })
}

pub async fn claim_one(
    db: &SqlitePool,
    player_id: i64,
    incr_id: i64,
) -> Result<(ReadMailReply, MailClaimOutcome), AppError> {
    let Some(mail) = mail::get_by_incr_id(db, player_id, incr_id).await? else {
        return Err(AppError::InvalidRequest);
    };

    let outcome = if mail.state == 1 {
        MailClaimOutcome {
            incr_ids: vec![incr_id],
            ..Default::default()
        }
    } else {
        claim_mails(db, player_id, vec![(mail.incr_id, mail.attachment)]).await?
    };

    Ok((
        ReadMailReply {
            incr_id: Some(incr_id as u64),
        },
        outcome,
    ))
}

pub async fn claim_batch(
    db: &SqlitePool,
    player_id: i64,
) -> Result<(ReadMailBatchReply, MailClaimOutcome), AppError> {
    let mails = mail::get_claimable(db, player_id)
        .await?
        .into_iter()
        .map(|mail| (mail.incr_id, mail.attachment))
        .collect::<Vec<_>>();

    let outcome = claim_mails(db, player_id, mails).await?;
    let reply = ReadMailBatchReply {
        incr_ids: outcome.incr_ids.iter().map(|id| *id as u64).collect(),
    };

    Ok((reply, outcome))
}

async fn claim_mails(
    db: &SqlitePool,
    player_id: i64,
    mails: Vec<(i64, String)>,
) -> Result<MailClaimOutcome, AppError> {
    if mails.is_empty() {
        return Ok(MailClaimOutcome::default());
    }

    let previous_mail_red_dot = red_dot::sync_mail_red_dot(db, player_id).await?;

    let mut rewards = reward::RewardSet::default();
    for (_, attachment) in &mails {
        rewards.extend(reward::parse(attachment));
    }
    let incr_ids = mails
        .iter()
        .map(|(incr_id, _)| *incr_id)
        .collect::<Vec<_>>();
    let material_changes = rewards.material_changes();
    let mut tx = db.begin().await?;
    mail::mark_claimed_in_transaction(&mut tx, player_id, &incr_ids).await?;
    let applied_rewards = reward::apply_in_transaction(&mut tx, db, player_id, rewards).await?;
    tx.commit().await?;
    let mail_red_dot = red_dot::sync_mail_red_dot(db, player_id).await?;

    Ok(MailClaimOutcome {
        incr_ids,
        rewards: applied_rewards,
        material_changes,
        mail_red_dot: (previous_mail_red_dot.0 != mail_red_dot.0).then_some(mail_red_dot),
    })
}

#[cfg(test)]
mod test;
