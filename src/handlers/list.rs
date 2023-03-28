use crate::error::Result;
use crate::utils::time::{LocaleDateTime, TimestampDisplay};
use trash::os_limited::list as trash_list;

pub fn list() -> Result<()> {
    let helper = LocaleDateTime::try_new()?;
    let mut items = trash_list()?;

    // From old to new along the top to the bottom.
    items.sort_by_key(|item| item.time_deleted);

    let list: String = items
        .into_iter()
        .map(|item| {
            let src = item.original_path();
            let src = src.to_string_lossy();
            let time = item.time_deleted.to_string(&helper);

            format!("{time} {src}\n")
        })
        .collect();

    print!("{list}");

    Ok(())
}
