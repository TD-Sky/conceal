use std::io::{stdout, BufWriter};

use crate::{
    error::Result,
    handler::list::{items, render},
    util::tui::confirm_or_no,
};

#[allow(unreachable_code)]
pub fn clean(all: bool) -> Result<()> {
    let items = items(all)?;

    if items.is_empty() {
        return Ok(());
    }

    render(&mut BufWriter::new(stdout()), &items)?;
    if !confirm_or_no("Clean above items?") {
        return Ok(());
    }

    #[cfg(all(
        unix,
        not(target_os = "macos"),
        not(target_os = "ios"),
        not(target_os = "android")
    ))]
    {
        use trash::os_limited::purge_all;
        purge_all(items)?;
    }
    #[cfg(target_os = "macos")]
    {
        // FIXME
        unimplemented!()
    }

    Ok(())
}
