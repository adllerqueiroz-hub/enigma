use crate::error::AppError;
use database::db::game::odyssey;
use sonettobuf::{
    OdysseyBagInfo, OdysseyElement, OdysseyFightInfo, OdysseyForm, OdysseyFormInfo,
    OdysseyGetInfoReply, OdysseyInfo, OdysseyItem, OdysseyMap, OdysseyMapInfo,
    OdysseyMercenaryInfo, OdysseyPropInfo, OdysseyReligionInfo, OdysseyTalentInfo,
    OdysseyTalentNode,
};
use sqlx::SqlitePool;

#[derive(Clone, Copy, Debug)]
pub struct OdysseyManager {
    player_id: i64,
}

impl OdysseyManager {
    pub fn new(player_id: i64) -> Self {
        Self { player_id }
    }

    pub async fn sync(self, db: &SqlitePool, tables: &config::GameDB) -> Result<(), AppError> {
        odyssey::sync_info(db, self.player_id, tables).await?;
        Ok(())
    }

    pub async fn info(self, db: &SqlitePool) -> Result<OdysseyGetInfoReply, AppError> {
        get_info(db, self.player_id).await
    }
}

async fn get_info(db: &SqlitePool, user_id: i64) -> Result<OdysseyGetInfoReply, AppError> {
    let rows = odyssey::get_info(db, user_id).await?;
    let state = rows.state;

    Ok(OdysseyGetInfoReply {
        info: Some(OdysseyInfo {
            prop_info: Some(OdysseyPropInfo {
                exp: Some(state.exp),
                level: Some(state.level),
                params: Some(state.params),
            }),
            map_info: Some(OdysseyMapInfo {
                curr_ele_id: Some(state.curr_element_id),
                maps: rows
                    .maps
                    .into_iter()
                    .map(|map| OdysseyMap {
                        id: Some(map.map_id),
                        explore_value: Some(map.explore_value),
                    })
                    .collect(),
                elements: rows
                    .elements
                    .into_iter()
                    .map(|element| OdysseyElement {
                        id: Some(element.element_id),
                        status: Some(element.status),
                        option_ele: None,
                        religion_ele: None,
                        conquest_ele: None,
                        mythic_ele: None,
                    })
                    .collect(),
                finished_ele_ids: Vec::new(),
            }),
            bag_info: Some(OdysseyBagInfo {
                items: rows
                    .items
                    .into_iter()
                    .map(|item| OdysseyItem {
                        uid: Some(item.uid),
                        id: Some(item.item_id),
                        count: Some(item.count),
                        new_flag: Some(item.new_flag),
                    })
                    .collect(),
            }),
            talent_info: Some(OdysseyTalentInfo {
                point: Some(state.talent_point),
                nodes: rows
                    .talents
                    .into_iter()
                    .map(|talent| OdysseyTalentNode {
                        id: Some(talent.node_id),
                        level: Some(talent.level),
                        consume: Some(talent.consume),
                    })
                    .collect(),
                cassandra_tree: Some(state.cassandra_tree),
            }),
            form_info: Some(OdysseyFormInfo {
                curr_form: Some(OdysseyForm {
                    no: Some(1),
                    heroes: Vec::new(),
                    suits: Vec::new(),
                    cloth_id: Some(0),
                }),
            }),
            fight_info: Some(OdysseyFightInfo {
                mercenary_info: Some(OdysseyMercenaryInfo {
                    next_ref_time: Some(state.next_mercenary_refresh),
                    suits: Vec::new(),
                }),
                religion_info: Some(OdysseyReligionInfo {
                    members: Vec::new(),
                }),
            }),
        }),
    })
}
