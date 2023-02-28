use crate::utils::time::{LocalDateTimeHelper, UnixTimestampToString};
use anyhow::Result;
use trash::os_limited::list as trash_list;

pub fn list() -> Result<()> {
    let local_date_time_helper = LocalDateTimeHelper::default();
    let mut items = trash_list()?;

    // From old to new along the top to the bottom.
    items.sort_by_key(|item| item.time_deleted);

    let list: String = items
        .into_iter()
        .map(|item| {
            let src = item.original_path();
            let src = src.to_string_lossy();
            let time = item.time_deleted.to_string(&local_date_time_helper);

            format!("{time} {src}\n")
        })
        .collect();

    print!("{list}");

    Ok(())
}
