use time::error::IndeterminateOffset;
use time::format_description::FormatItem;
use time::macros::format_description;
use time::OffsetDateTime;
use time::UtcOffset;

pub struct LocaleDateTime {
    local_offset: UtcOffset,
    format: &'static [FormatItem<'static>],
}

pub trait TimestampDisplay {
    fn to_string(self, helper: &LocaleDateTime) -> String;
}

impl TimestampDisplay for i64 {
    fn to_string(self, helper: &LocaleDateTime) -> String {
        OffsetDateTime::from_unix_timestamp(self)
            .unwrap()
            .to_offset(helper.local_offset)
            .format(helper.format)
            .unwrap()
    }
}

impl LocaleDateTime {
    pub fn try_new() -> Result<Self, IndeterminateOffset> {
        Ok(Self {
            local_offset: UtcOffset::current_local_offset()?,
            format: format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
        })
    }
}
