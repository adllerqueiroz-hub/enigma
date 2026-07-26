use crate::error::AppError;
use database::db::game::activity_state::{self, ActivityStateKind, ActivityStateSet};
use sonettobuf::{
    FairylandInfo, GetFairylandInfoReply, RecordDialogReply, RecordElementReply, ResolvePuzzleReply,
};
use sqlx::SqlitePool;

const FAIRYLAND_SCOPE_ID: i32 = 0;

pub async fn get_info(db: &SqlitePool, player_id: i64) -> Result<GetFairylandInfoReply, AppError> {
    Ok(GetFairylandInfoReply {
        info: Some(snapshot(db, player_id).await?),
    })
}

pub async fn record_dialog(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    dialog_id: i32,
) -> Result<RecordDialogReply, AppError> {
    if !tables
        .fairyland_puzzle_talk
        .iter()
        .any(|talk| talk.id == dialog_id)
    {
        return Err(AppError::InvalidRequest);
    }
    record(db, player_id, ActivityStateKind::FairylandDialog, dialog_id).await?;
    Ok(RecordDialogReply {
        info: Some(snapshot(db, player_id).await?),
    })
}

pub async fn record_element(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    element_id: i32,
) -> Result<RecordElementReply, AppError> {
    tables
        .fairy_land_element
        .get(element_id)
        .ok_or(AppError::InvalidRequest)?;
    record(
        db,
        player_id,
        ActivityStateKind::FairylandElement,
        element_id,
    )
    .await?;
    Ok(RecordElementReply {
        info: Some(snapshot(db, player_id).await?),
    })
}

pub async fn resolve_puzzle(
    db: &SqlitePool,
    tables: &config::GameDB,
    player_id: i64,
    puzzle_id: i32,
    answer: &str,
) -> Result<ResolvePuzzleReply, AppError> {
    let puzzle = tables
        .fairyland_puzzle
        .get(puzzle_id)
        .ok_or(AppError::InvalidRequest)?;
    if puzzle.answer != answer {
        return Err(AppError::InvalidRequest);
    }
    record(db, player_id, ActivityStateKind::FairylandPuzzle, puzzle_id).await?;
    Ok(ResolvePuzzleReply {
        info: Some(snapshot(db, player_id).await?),
    })
}

async fn record(
    db: &SqlitePool,
    player_id: i64,
    kind: ActivityStateKind,
    entry_id: i32,
) -> Result<(), AppError> {
    activity_state::set(
        db,
        player_id,
        FAIRYLAND_SCOPE_ID,
        ActivityStateSet {
            kind,
            entry_id,
            state: 1,
            progress: 0,
            ext: "",
        },
    )
    .await?;
    Ok(())
}

async fn snapshot(db: &SqlitePool, player_id: i64) -> Result<FairylandInfo, AppError> {
    Ok(FairylandInfo {
        pass_puzzle_id: completed_ids(
            activity_state::get(
                db,
                player_id,
                FAIRYLAND_SCOPE_ID,
                ActivityStateKind::FairylandPuzzle,
            )
            .await?,
        ),
        dialog_id: completed_ids(
            activity_state::get(
                db,
                player_id,
                FAIRYLAND_SCOPE_ID,
                ActivityStateKind::FairylandDialog,
            )
            .await?,
        ),
        finish_element_id: completed_ids(
            activity_state::get(
                db,
                player_id,
                FAIRYLAND_SCOPE_ID,
                ActivityStateKind::FairylandElement,
            )
            .await?,
        ),
    })
}

fn completed_ids(states: activity_state::ActivityStates) -> Vec<i32> {
    let mut ids = states
        .into_iter()
        .filter_map(|(id, (state, _, _))| (state != 0).then_some(id))
        .collect::<Vec<_>>();
    ids.sort_unstable();
    ids
}

#[cfg(test)]
mod test;
