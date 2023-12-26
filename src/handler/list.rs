use std::env;
use std::fmt::Write;

use owo_colors::OwoColorize;
use trash::TrashItem;

use crate::error::Result;
use crate::util;
use crate::util::time::local_datetime;

pub fn list(all: bool) -> Result<()> {
    items(all).map(|items| print!("{}", render(&items)))
}

pub(super) fn items(all: bool) -> Result<Vec<TrashItem>> {
    let pwd = (!all).then(env::current_dir).transpose()?;
    Ok(util::trash::list(pwd.as_deref())?)
}

pub(super) fn render<'a>(items: impl IntoIterator<Item = &'a TrashItem>) -> String {
    let mut list = String::new();

    for item in items {
        let time = local_datetime(item.time_deleted).format(util::time::FORMAT);
        let src = item.original_path();
        let _ = writeln!(
            list,
            "{time} {src}",
            time = time.bright_yellow(),
            src = src.to_string_lossy()
        );
    }

    list
}
