pub mod time {
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
}

pub mod tui {
    use std::io;
    use std::io::{stdin, stdout};
    use std::io::{Read, Write};

    pub fn confirm(prompt: &str) -> bool {
        let mut stdout = stdout();
        || -> io::Result<()> {
            stdout.write_all(prompt.as_bytes())?;
            stdout.flush()
        }()
        .expect("Prompt failure");

        stdin()
            .bytes()
            .next()
            .and_then(|c| c.ok())
            .map(|c| c == b'y' || c == b'Y')
            .unwrap_or(false)
    }
}

pub mod trash {
    use std::path::Path;

    use trash::os_limited::list as trash_list;
    use trash::TrashItem;

    pub fn list(prefix: Option<&Path>) -> Result<Vec<TrashItem>, trash::Error> {
        let mut items = trash_list()?;

        if let Some(prefix) = prefix {
            items.retain_mut(|item| {
                if let Ok(p) = item.original_parent.strip_prefix(prefix) {
                    item.original_parent = p.to_owned();
                    true
                } else {
                    false
                }
            })
        }
        items.sort_by_key(|item| item.time_deleted);

        Ok(items)
    }
}
