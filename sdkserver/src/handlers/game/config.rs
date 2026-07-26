use crate::models::response::GameConfigRsp;
use axum::response::Json;
use common::time::ServerTime;

pub async fn get() -> Json<GameConfigRsp> {
    let time = ServerTime::now_ms() as u128;
    let rsp = GameConfigRsp::with_timestamp(time);
    Json(rsp)
}
