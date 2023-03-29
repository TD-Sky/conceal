use std::env;

use crate::error::Result;
use crate::utils::time::{LocaleDateTime, TimestampDisplay};
use owo_colors::OwoColorize;
use std::path::Path;
use trash::os_limited::list as trash_list;

pub fn list(all: bool) -> Result<()> {
    let helper = LocaleDateTime::try_new()?;
    let mut items = trash_list()?;

    let list: String = if all {
        // From old to new along the top to the bottom.
        items.sort_by_key(|item| item.time_deleted);
        items
            .iter()
            .map(|item| {
                let src = &item.original_path();
                render(item.time_deleted, src, &helper)
            })
            .collect()
    } else {
        let pwd = env::current_dir()?;

        let mut items: Vec<_> = items
            .iter()
            .filter_map(|item| {
                Some((
                    item.time_deleted,
                    item.original_parent
                        .strip_prefix(&pwd)
                        .ok()?
                        .join(&item.name),
                ))
            })
            .collect();
        items.sort_by_key(|(time_deleted, _)| *time_deleted);

        items
            .iter()
            .map(|(time, src)| render(*time, src, &helper))
            .collect()
    };

    print!("{list}");

    Ok(())
}

fn render(time: i64, src: &Path, locale: &LocaleDateTime) -> String {
    let time = time.to_string(locale);
    let src = src.to_string_lossy();
    format!("{time} {src}\n", time = time.bright_yellow())
}
