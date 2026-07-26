use chrono::{NaiveDateTime, TimeZone, Utc};

pub(crate) fn goods_store_id(value: &str) -> Option<i32> {
    value.parse().ok()
}

pub(crate) fn is_time_active(online_time: &str, offline_time: &str, now: i64) -> bool {
    let online = parse_time_millis(online_time);
    let offline = parse_time_millis(offline_time);

    (online == 0 || online <= now) && (offline == 0 || now <= offline)
}

pub(crate) fn parse_time_millis(value: &str) -> i64 {
    NaiveDateTime::parse_from_str(value.trim(), "%Y-%m-%d %H:%M:%S")
        .map(|time| Utc.from_utc_datetime(&time).timestamp_millis())
        .unwrap_or(0)
}
