use common::time::ServerTime;
use sonettobuf::GetServerTimeReply;

pub fn server_time_reply() -> GetServerTimeReply {
    GetServerTimeReply {
        server_time: Some(ServerTime::now_ms() as u64),
        offset_time: Some(ServerTime::server_utc_offset_ms()),
    }
}
