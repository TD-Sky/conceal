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

    let prompt = render(&items) + "\nClean above items? (y/n) ";
    if !confirm(&prompt) {
        return Ok(());
    }

    purge_all(items)?;

    Ok(())
}
