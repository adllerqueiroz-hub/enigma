use sonettobuf::{Fight, FightEntityInfo, FightTeam};

use crate::engine::manager::BattleManagers;

#[test]
fn ulrich_channels_project_configured_enemy_and_ally_outputs() {
    crate::test_support::init_config();
    let entity = |uid, team_type| FightEntityInfo {
        uid: Some(uid),
        current_hp: Some(100),
        team_type: Some(team_type),
        ..Default::default()
    };
    let fight = Fight {
        attacker: Some(FightTeam {
            entitys: vec![entity(10, 1), entity(11, 1)],
            ..Default::default()
        }),
        defender: Some(FightTeam {
            entitys: vec![entity(-1, 2), entity(-2, 2)],
            ..Default::default()
        }),
        ..Default::default()
    };
    let mut managers = BattleManagers::seeded(&fight);
    managers.buff.add(&managers.hp, 10, 10, 31070111, 0);
    managers.buff.add(&managers.hp, 10, 10, 31070121, 0);
    managers.buff.add_special_count(10, &[31070111], 3);
    managers.buff.add_special_count(10, &[31070121], 3);

    let outputs = managers.buff.special_count_outputs(&managers.hp);

    assert_eq!(
        outputs
            .iter()
            .map(|output| (output.target_uid, output.output_buff_id, output.amount))
            .collect::<Vec<_>>(),
        vec![
            (-1, 31070141, -420),
            (-2, 31070141, -420),
            (10, 31070151, 420),
            (11, 31070151, 420),
        ]
    );
}
