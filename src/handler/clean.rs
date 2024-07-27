use std::io::{stdout, BufWriter};

use trash::os_limited::purge_all;

use crate::{
    error::Result,
    handler::list::{items, render},
    util::tui::confirm,
};

pub fn clean(all: bool) -> Result<()> {
    let items = items(all)?;

    if items.is_empty() {
        return Ok(());
    }

    render(&mut BufWriter::new(stdout()), &items)?;
    if !confirm("\nClean above items? (y/n) ") {
        return Ok(());
    }

    purge_all(items)?;

    Ok(())
}
