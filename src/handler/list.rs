use std::env;
use std::io::{self, stdout, BufWriter, Write};

use owo_colors::OwoColorize;
use trash::TrashItem;

use crate::{
    error::Result,
    util::{self, time::local_datetime},
};

pub fn list(all: bool) -> Result<()> {
    let items = items(all)?;
    render(&mut BufWriter::new(stdout()), &items)?;
    Ok(())
}

pub(in crate::handler) fn items(all: bool) -> Result<Vec<TrashItem>> {
    let pwd = (!all).then(env::current_dir).transpose()?;
    Ok(util::trash::list(pwd.as_deref())?)
}

pub(in crate::handler) fn render<'a>(
    w: &mut dyn Write,
    items: impl IntoIterator<Item = &'a TrashItem>,
) -> io::Result<()> {
    for item in items {
        let time = local_datetime(item.time_deleted).format(util::time::FORMAT);
        let src = item.original_path();
        writeln!(
            w,
            "{time} {src}",
            time = time.bright_yellow(),
            src = src.display()
        )?;
    }

    w.flush()
}
