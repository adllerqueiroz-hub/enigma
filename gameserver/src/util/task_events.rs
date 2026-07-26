use crate::{error::AppError, net::context::ConnectionContext};
use database::db::game::{
    achievements, player_card,
    tasks::{self, TaskEvent},
};
use sonettobuf::{CmdId, UpdateAchievementPush, UpdateTaskPush};

use super::push;

pub async fn notify_tasks(
    ctx: &mut ConnectionContext,
    tasks: Vec<database::models::game::tasks::UserTask>,
) -> Result<(), AppError> {
    if !tasks.is_empty() {
        let player_id = ctx.player()?.id;
        let red_dot_types = recurring_red_dot_types(
            tasks
                .iter()
                .map(|task| (task.type_id, task.has_finished, task.finish_count)),
        );
        let activity_info = tasks::list_activity(ctx.state.db, player_id, Vec::new())
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        ctx.notify(
            CmdId::UpdateTaskPushCmd,
            UpdateTaskPush {
                task_info: tasks.into_iter().map(Into::into).collect(),
                activity_info,
            },
        )
        .await?;
        notify_task_red_dots(ctx, player_id, red_dot_types).await?;
    }
    Ok(())
}

pub async fn notify_task_red_dots(
    ctx: &mut ConnectionContext,
    player_id: i64,
    type_ids: Vec<i32>,
) -> Result<(), AppError> {
    for type_id in type_ids {
        let Some((task_type, define_id)) = task_red_dot_route(type_id) else {
            continue;
        };
        let expiry = tasks::claimable_expiry(ctx.state.db, player_id, task_type).await?;
        push::send_red_dot_value_push(
            ctx,
            define_id,
            vec![0],
            false,
            i32::from(expiry.is_some()),
            expiry.unwrap_or_default(),
        )
        .await?;
    }
    Ok(())
}

pub fn recurring_red_dot_types(tasks: impl IntoIterator<Item = (i32, bool, i32)>) -> Vec<i32> {
    let mut type_ids = tasks
        .into_iter()
        .filter(|(_, has_finished, finish_count)| *has_finished || *finish_count > 0)
        .filter_map(|(type_id, _, _)| task_red_dot_route(type_id).map(|_| type_id))
        .collect::<Vec<_>>();
    type_ids.sort_unstable();
    type_ids.dedup();
    type_ids
}

fn task_red_dot_route(type_id: i32) -> Option<(tasks::TaskType, i32)> {
    match tasks::TaskType::from_id(type_id)? {
        tasks::TaskType::Daily => Some((
            tasks::TaskType::Daily,
            crate::types::red_dot_id::RedDotId::DailyTask.id(),
        )),
        tasks::TaskType::Weekly => Some((
            tasks::TaskType::Weekly,
            crate::types::red_dot_id::RedDotId::WeeklyTask.id(),
        )),
        _ => None,
    }
}

pub async fn notify(
    ctx: &mut ConnectionContext,
    player_id: i64,
    event: TaskEvent,
) -> Result<(), AppError> {
    let updated_tasks = tasks::sync_event_tasks(ctx.state.db, player_id, event).await?;
    let updated_achievements = achievements::sync_event(ctx.state.db, player_id, event).await?;

    notify_tasks(ctx, updated_tasks).await?;

    if !updated_achievements.is_empty() {
        ctx.notify(
            CmdId::UpdateAchievementPushCmd,
            UpdateAchievementPush {
                infos: updated_achievements.into_iter().map(Into::into).collect(),
            },
        )
        .await?;
    }

    if let Some(count) = event.hero_touch_count() {
        player_card::increment_hero_cover_times(ctx.state.db, player_id, count).await?;
        crate::util::push::send_player_card_info_push(ctx, player_id).await?;
    }

    Ok(())
}
