use std::env;
use std::path::Path;

use owo_colors::OwoColorize;
use trash::os_limited::list as trash_list;

use crate::error::Result;
use crate::util::time::{self, local_datetime};

pub fn list(all: bool) -> Result<()> {
    let mut items = trash_list()?;

    let list: String = if all {
        // From old to new along the top to the bottom.
        items.sort_by_key(|item| item.time_deleted);
        items
            .iter()
            .map(|item| {
                let src = &item.original_path();
                render(item.time_deleted, src)
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

        items.iter().map(|(time, src)| render(*time, src)).collect()
    };

    print!("{list}");

    Ok(())
}

fn render(time: i64, src: &Path) -> String {
    let time = local_datetime(time).format(time::FORMAT);
    let src = src.to_string_lossy();
    format!("{time} {src}\n", time = time.bright_yellow())
}
