use super::*;

pub(super) fn distribute_buff_ops(
    context: &BehaviorOpContext<'_>,
    behavior: &ParsedBehavior,
) -> Option<Vec<RuleOp>> {
    let [source_buff_id, output_buff_id] = behavior.args.as_slice() else {
        return None;
    };
    if *source_buff_id <= 0 || *output_buff_id <= 0 {
        return None;
    }
    Some(vec![RuleOp::Command(BattleCommand::Buff(
        BuffCommand::Convert(BuffConvert {
            origin: super::command_origin(behavior)?,
            source_uid: context.source_uid,
            target_uid: context.target_uid,
            source_buff_id: *source_buff_id,
            output_buff_id: *output_buff_id,
        }),
    ))])
}
