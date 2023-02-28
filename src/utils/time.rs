use time::format_description::FormatItem;
use time::macros::format_description;
use time::OffsetDateTime;
use time::UtcOffset;

pub struct LocalDateTimeHelper {
    local_offset: UtcOffset,
    format: &'static [FormatItem<'static>],
}

pub trait UnixTimestampToString {
    fn to_string(self, helper: &LocalDateTimeHelper) -> String;
}

impl UnixTimestampToString for i64 {
    fn to_string(self, helper: &LocalDateTimeHelper) -> String {
        OffsetDateTime::from_unix_timestamp(self)
            .unwrap()
            .to_offset(helper.local_offset)
            .format(&helper.format)
            .unwrap()
    }
}

impl Default for LocalDateTimeHelper {
    fn default() -> Self {
        Self {
            local_offset: UtcOffset::current_local_offset().unwrap(),
            format: format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
        }
    }
}
