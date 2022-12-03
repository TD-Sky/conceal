use anyhow::Result;
use chrono::{Local, TimeZone};
use trash::os_limited::list as trash_list;

pub fn list() -> Result<()> {
    let mut items = trash_list()?;

    // From old to new along the top to the bottom.
    items.sort_by_key(|item| item.time_deleted);

    let list: String = items
        .into_iter()
        .map(|item| {
            let src = item.original_path();
            let src = src.to_string_lossy();

            let time = Local.timestamp_opt(item.time_deleted, 0).unwrap();
            let time = time.format("%Y-%m-%d %H:%M:%S");

            format!("{time} {src}\n")
        })
        .collect();

    Ok(print!("{list}"))
}
