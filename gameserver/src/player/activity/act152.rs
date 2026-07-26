use super::*;
use chrono::{Duration, NaiveDate, NaiveDateTime, TimeZone, Utc};
use sonettobuf::{Act152AcceptPresentReply, Get152InfoReply, MaterialData, PresentInfo};

pub struct Act152PresentClaim {
    pub reply: Act152AcceptPresentReply,
    pub rewards: Option<reward::AppliedRewards>,
    pub material_changes: Vec<(u32, u32, i32)>,
}

pub async fn act152_info(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
) -> Result<Get152InfoReply, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act152Present).await?;

    Ok(Get152InfoReply {
        activity_id: Some(activity_id),
        present_ids: states
            .iter()
            .filter_map(|(id, (state, _, _))| (*state != 0).then_some(*id))
            .collect(),
    })
}

pub async fn accept_act152_present(
    db: &SqlitePool,
    player_id: i64,
    activity_id: Option<i32>,
    present_id: Option<i32>,
) -> Result<Act152PresentClaim, AppError> {
    let activity_id = resolve_activity_id(activity_id)?;
    let present_id = present_id.ok_or(AppError::InvalidRequest)?;
    let row = config::configs::get()
        .activity152
        .iter()
        .find(|row| row.activity_id == activity_id && row.present_id == present_id)
        .ok_or(AppError::InvalidRequest)?;

    if !is_unlocked(&row.accept_date) {
        return Err(AppError::InvalidRequest);
    }

    let states =
        activity_state::get(db, player_id, activity_id, ActivityStateKind::Act152Present).await?;
    if states
        .get(&present_id)
        .is_some_and(|(state, _, _)| *state != 0)
    {
        return Ok(Act152PresentClaim {
            reply: reply(activity_id, present_id, Vec::new()),
            rewards: None,
            material_changes: Vec::new(),
        });
    }

    let parsed = reward::parse(&row.bonus);
    let material_changes = parsed.material_changes();
    let bonuses = material_changes
        .iter()
        .map(|(materil_type, materil_id, quantity)| MaterialData {
            materil_type: Some(*materil_type),
            materil_id: Some(*materil_id),
            quantity: Some(*quantity),
        })
        .collect();
    let rewards = reward::apply(db, player_id, parsed).await?;

    activity_state::set(
        db,
        player_id,
        activity_id,
        ActivityStateSet {
            kind: ActivityStateKind::Act152Present,
            entry_id: present_id,
            state: 1,
            progress: 0,
            ext: "",
        },
    )
    .await?;

    Ok(Act152PresentClaim {
        reply: reply(activity_id, present_id, bonuses),
        rewards: Some(rewards),
        material_changes,
    })
}

fn resolve_activity_id(activity_id: Option<i32>) -> Result<i32, AppError> {
    activity_id
        .or_else(|| {
            config::configs::get()
                .activity152
                .iter()
                .map(|row| row.activity_id)
                .max()
        })
        .ok_or(AppError::InvalidRequest)
}

fn reply(
    activity_id: i32,
    present_id: i32,
    bonuses: Vec<MaterialData>,
) -> Act152AcceptPresentReply {
    Act152AcceptPresentReply {
        activity_id: Some(activity_id),
        present: Some(PresentInfo {
            present_id: Some(present_id),
            bonuses,
        }),
    }
}

fn is_unlocked(accept_date: &str) -> bool {
    parse_accept_date_ms(accept_date)
        .is_none_or(|unlock_ms| common::time::ServerTime::now_ms() >= unlock_ms)
}

fn parse_accept_date_ms(value: &str) -> Option<i64> {
    if let Ok(date) = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S") {
        return Some(Utc.from_utc_datetime(&date).timestamp_millis());
    }

    let (date, time) = value.split_once(' ')?;
    let time = time.strip_prefix("24:")?;
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
    let next = date.checked_add_signed(Duration::days(1))?;
    let date = NaiveDateTime::parse_from_str(
        &format!("{} 00:{time}", next.format("%Y-%m-%d")),
        "%Y-%m-%d %H:%M:%S",
    )
    .ok()?;
    Some(Utc.from_utc_datetime(&date).timestamp_millis())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_midnight_24_hour_accept_date() {
        assert_eq!(
            parse_accept_date_ms("2024-05-30 24:00:00"),
            parse_accept_date_ms("2024-05-31 00:00:00")
        );
    }
}
