use std::io::{stdout, BufWriter};

use trash::os_limited::purge_all;

use crate::{
    error::Result,
    handler::list::{items, render},
    util::tui::confirm_or_no,
};

pub fn clean(all: bool) -> Result<()> {
    let items = items(all)?;

    if items.is_empty() {
        return Ok(());
    }

    render(&mut BufWriter::new(stdout()), &items)?;
    if !confirm_or_no("Clean above items?") {
        return Ok(());
    }

    purge_all(items)?;

    Ok(())
}
