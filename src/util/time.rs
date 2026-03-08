use chrono::DateTime;
use chrono::Local;
use chrono_systemd_time::parse_timestamp_tz;
use trash::TrashItem;

use crate::Error;

pub const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn local_datetime(unix_timestamp: i64) -> DateTime<Local> {
    DateTime::from_timestamp(unix_timestamp, 0)
        .map(|utc| utc.with_timezone(&Local))
        .unwrap()
}

pub fn retain_during(
    items: &mut Vec<TrashItem>,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<(), Error> {
    let since = since
        .map(|s| parse_timestamp_tz(s, Local).map(|t| t.latest().timestamp()))
        .transpose()?;
    let until = until
        .map(|s| parse_timestamp_tz(s, Local).map(|t| t.earliest().timestamp()))
        .transpose()?;

    let f: &dyn Fn(&TrashItem) -> bool = match (since, until) {
        (None, None) => return Ok(()),
        (Some(since), Some(until)) => {
            if since >= until {
                return Err(Error::Msg("`since` should be less than `until`"));
            }

            &(move |item| since <= item.time_deleted && item.time_deleted < until)
        }
        (Some(since), None) => &(move |item| since <= item.time_deleted),
        (None, Some(until)) => &(move |item| item.time_deleted < until),
    };

    items.retain(f);

    Ok(())
}
