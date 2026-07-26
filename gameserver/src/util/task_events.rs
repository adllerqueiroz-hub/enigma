use crate::{error::AppError, net::context::ConnectionContext};
use logic::task::{TaskEvent, UserTask};
use sonettobuf::{CmdId, UpdateAchievementPush, UpdateTaskPush};

use super::push;

pub async fn notify_tasks(
    ctx: &mut ConnectionContext,
    tasks: Vec<UserTask>,
) -> Result<(), AppError> {
    if !tasks.is_empty() {
        let red_dot_types = logic::task::recurring_red_dot_types(
            tasks
                .iter()
                .map(|task| (task.type_id, task.has_finished, task.finish_count)),
        );
        let activity_info = ctx.player()?.tasks.activity_info(ctx.state.db).await?;
        ctx.notify(
            CmdId::UpdateTaskPushCmd,
            UpdateTaskPush {
                task_info: tasks.into_iter().map(Into::into).collect(),
                activity_info,
            },
        )
        .await?;
        notify_task_red_dots(ctx, red_dot_types).await?;
    }
    Ok(())
}

pub async fn notify_task_red_dots(
    ctx: &mut ConnectionContext,
    type_ids: Vec<i32>,
) -> Result<(), AppError> {
    for type_id in type_ids {
        let Some(red_dot) = ctx
            .player()?
            .tasks
            .recurring_red_dot(ctx.state.db, type_id)
            .await?
        else {
            continue;
        };
        push::send_red_dot_value_push(
            ctx,
            red_dot.define_id,
            vec![0],
            false,
            red_dot.value,
            red_dot.expiry,
        )
        .await?;
    }
    Ok(())
}

pub async fn notify(
    ctx: &mut ConnectionContext,
    player_id: i64,
    event: TaskEvent,
) -> Result<(), AppError> {
    let db = ctx.state.db;
    let updated_tasks = ctx.player_mut()?.tasks.sync_event(db, event).await?;
    let updated_achievements = ctx
        .player_mut()?
        .collection
        .sync_task_event(db, event)
        .await?;

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
        ctx.player()?
            .profile
            .increment_hero_cover_times(ctx.state.db, count)
            .await?;
        crate::util::push::send_player_card_info_push(ctx, player_id).await?;
    }

    Ok(())
}
