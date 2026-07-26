use super::*;

pub(super) fn copy_status_ops(
    context: &mut BehaviorOpContext<'_>,
    behavior: &ParsedBehavior,
) -> Option<Vec<RuleOp>> {
    let [count, status_id] = behavior.args.as_slice() else {
        return None;
    };
    let status = BuffStatus::from_id(*status_id);
    if *count <= 0 || status == BuffStatus::Unknown {
        return None;
    }

    let mut candidates = context
        .managers
        .buff
        .buffs_with_status(context.source_uid, status);
    let take = if *count == 99 {
        candidates.len()
    } else {
        usize::try_from(*count).ok()?.min(candidates.len())
    };
    let origin = super::command_origin(behavior)?;
    let mut ops = Vec::with_capacity(take);
    for _ in 0..take {
        let index = if *count == 99 {
            0
        } else {
            let ids = candidates
                .iter()
                .map(|(buff_id, _)| *buff_id)
                .collect::<Vec<_>>();
            context
                .determinism
                .take_random_buff(&ids)
                .and_then(|buff_id| ids.iter().position(|candidate| *candidate == buff_id))
                .or_else(|| context.determinism.lua_random_index(candidates.len()))?
        };
        let (buff_id, amount) = candidates.remove(index);
        ops.push(RuleOp::Command(BattleCommand::Buff(BuffCommand::Grant(
            BuffGrant {
                origin,
                source_uid: context.source_uid,
                target_uid: context.target_uid,
                buff_id,
                amount: Some(amount),
                occurrences: 1,
                child_uid_reservations: 0,
            },
        ))));
    }
    Some(ops)
}

pub(in crate::engine::skill::behavior) fn supports_status_copy(behavior: &ParsedBehavior) -> bool {
    matches!(behavior.args.as_slice(), [count, status] if *count > 0
        && BuffStatus::from_id(*status) != BuffStatus::Unknown)
}
