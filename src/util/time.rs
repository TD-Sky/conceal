use chrono::DateTime;
use chrono::Local;

pub const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn local_datetime(unix_timestamp: i64) -> DateTime<Local> {
    DateTime::from_timestamp(unix_timestamp, 0)
        .map(|utc| utc.with_timezone(&Local))
        .unwrap()
}
