use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::TimeZone;

pub const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn local_datetime(unix_timestamp: i64) -> DateTime<Local> {
    NaiveDateTime::from_timestamp_opt(unix_timestamp, 0)
        .map(|nt| Local.from_utc_datetime(&nt))
        .unwrap()
}
