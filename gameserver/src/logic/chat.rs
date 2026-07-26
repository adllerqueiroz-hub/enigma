use crate::error::AppError;
use common::time::ServerTime;
use database::db::game::player_infos;
use sonettobuf::{
    ChatMsg, ChatMsgPush, DeleteOfflineMsgReply, ReportReply, SendMsgReply, SendMsgRequest,
    WordTestReply,
};
use sqlx::SqlitePool;

pub async fn send_msg(
    db: &SqlitePool,
    player_id: i64,
    sequence: u32,
    req: SendMsgRequest,
) -> Result<(ChatMsgPush, SendMsgReply), AppError> {
    let (sender_name, level) = player_infos::get_user_basic_info(db, player_id)
        .await
        .map(|(name, level, _)| (name, level as u32))?;
    let portrait = player_infos::get_player_info(db, player_id)
        .await?
        .map(|info| info.portrait as u32)
        .unwrap_or(0);
    let content = req.content.unwrap_or_default();
    let ext_data = req.ext_data.unwrap_or_default();
    let channel_type = req.channel_type.unwrap_or_default();
    let msg_type = req.msg_type.unwrap_or_default();
    let now = ServerTime::now_ms() as u64;

    Ok((
        ChatMsgPush {
            msg: vec![ChatMsg {
                msg_id: Some(now * 1000 + sequence as u64),
                channel_type: Some(channel_type),
                sender_id: Some(player_id as u64),
                sender_name: Some(sender_name),
                portrait: Some(portrait),
                content: Some(content.clone()),
                send_time: Some(now),
                level: Some(level),
                recipient_id: req.recipient_id,
                msg_type: Some(msg_type),
                ext_data: Some(ext_data.clone()),
            }],
        },
        SendMsgReply {
            message: None,
            content: Some(content),
            channel_type: Some(channel_type),
            msg_type: Some(msg_type),
            ext_data: Some(ext_data),
        },
    ))
}

pub fn delete_offline_msg() -> DeleteOfflineMsgReply {
    DeleteOfflineMsgReply {}
}

pub fn report() -> ReportReply {
    ReportReply {}
}

pub fn word_test() -> WordTestReply {
    WordTestReply {}
}
