use jiff::{
    Timestamp, Zoned,
    fmt::strtime::{self, BrokenDownTime},
    tz::TimeZone,
};
use parse_datetime::{ParseDateTimeError, parse_datetime};
use trash::TrashItem;

use crate::{Error, Result};

pub fn local_datetime(unix_timestamp: i64) -> Zoned {
    Timestamp::from_second(unix_timestamp)
        .map(|ts| ts.to_zoned(TimeZone::system()))
        .unwrap()
}

pub fn format_time(time: impl Into<BrokenDownTime>) -> String {
    strtime::format("%Y-%m-%d %H:%M:%S", time).unwrap()
}

pub fn retain_during(
    items: &mut Vec<TrashItem>,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<()> {
    let f = |s: &str| -> Result<_, ParseDateTimeError> {
        let t = parse_datetime(s)?;
        let secs = t
            .as_zoned()
            .ok_or(ParseDateTimeError::InvalidInput)?
            .timestamp()
            .as_second();
        Ok(secs)
    };

    let since = since.map(f).transpose()?;
    let until = until.map(f).transpose()?;

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
