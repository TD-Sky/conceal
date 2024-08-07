pub mod time {
    use chrono::DateTime;
    use chrono::Local;

    pub const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn local_datetime(unix_timestamp: i64) -> DateTime<Local> {
        DateTime::from_timestamp(unix_timestamp, 0)
            .map(|utc| utc.with_timezone(&Local))
            .unwrap()
    }
}

pub mod tui {
    use std::io::{self, stdin, stdout, Read, Write};

    pub fn confirm(prompt: &str) -> bool {
        || -> io::Result<()> {
            let mut stdout = stdout();
            stdout.write_all(prompt.as_bytes())?;
            stdout.flush()
        }()
        .expect("prompt failure");

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
                    item.original_parent = p.into();
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
