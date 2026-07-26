use crate::engine::{
    manager::BattleManagers,
    skill::{
        behavior::classify::BehaviorKind,
        buff_act::registry::BuffActKind,
        condition::{parse::ParsedConditionKind, registry::BehaviorOwnership},
        effect::SkillEffectSlot,
    },
};

pub fn behavior_is_owned_by_buff_act(
    slot: &SkillEffectSlot,
    source_uid: i64,
    managers: &BattleManagers,
) -> bool {
    if slot.behavior.spec.key.opcode != 1
        || slot.behavior.spec.key.type_name != "AddBuff"
        || slot.behavior.spec.kind != BehaviorKind::AddBuff
    {
        return false;
    }
    let Some(output_buff_id) = slot.behavior.arg(0) else {
        return false;
    };
    let counted_buff_ids = slot.conditions.iter().find_map(|condition| {
        let definition = crate::engine::skill::condition::registry::find_key(
            condition.opcode,
            &condition.type_name,
        )?;
        if definition.behavior_ownership != BehaviorOwnership::MatchingBuffAct {
            return None;
        }
        match &condition.kind {
            ParsedConditionKind::BuffIdCount { buff_ids, .. } => Some(buff_ids.as_slice()),
            _ => None,
        }
    });
    let Some(counted_buff_ids) = counted_buff_ids else {
        return false;
    };
    let Some(team) = managers.buff.team_type(source_uid) else {
        return false;
    };
    managers
        .buff
        .active_features(&managers.hp)
        .into_iter()
        .any(|feature| {
            feature.owner_alive
                && feature.team_type == team
                && counted_buff_ids.contains(&feature.buff_id)
                && crate::engine::skill::buff_act::is_kind(&feature, BuffActKind::AddToTarget)
                && feature
                    .values
                    .get(2..)
                    .is_some_and(|ids| ids.contains(&output_buff_id))
        })
}
