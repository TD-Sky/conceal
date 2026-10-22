#[cfg(any(freedesktop, target_os = "windows"))]
use std::io::{BufWriter, stdout};
use std::path::Path;

use crate::error::Result;
#[cfg(any(freedesktop, target_os = "windows"))]
use crate::{
    cli::conceal::{CleanArgs, DeleteArgs, ListArgs, RestoreArgs},
    utils::{
        trash::{items, render, select_items},
        tui::{confirm_or_no, confirm_or_yes},
    },
};

pub fn put(items: &[impl AsRef<Path>]) -> Result<()> {
    // Function `delete_all` wouldn't fail even if no file is specified.
    // But this doesn't meet our expectation.
    if items.is_empty() {
        return Err("Please specify the files to trash".into());
    }

    #[cfg(any(freedesktop, target_os = "windows"))]
    {
        trash::delete_all(items)?;
    }

    #[cfg(target_os = "macos")]
    {
        use trash::{
            TrashContext,
            macos::{DeleteMethod, TrashContextExtMacos},
        };
        let mut ctx = TrashContext::default();
        ctx.set_delete_method(DeleteMethod::NsFileManager);
        ctx.delete_all(items)?;
    }

    Ok(())
}

#[cfg(any(freedesktop, target_os = "windows"))]
pub fn restore(args: &RestoreArgs) -> Result<()> {
    // Users only can restore files discarded under the current directory.
    let items = select_items(args.finder.cmd(), false)?;

    if !items.is_empty() && confirm_or_yes("Restore above items?") {
        trash::os_limited::restore_all(items)?;
    }

    Ok(())
}

#[cfg(any(freedesktop, target_os = "windows"))]
pub fn list(args: &ListArgs) -> Result<()> {
    use crate::utils::time;

    let mut items = items(args.all)?;
    time::retain_during(&mut items, args.since.as_deref(), args.until.as_deref())?;

    render(&mut BufWriter::new(stdout()), &items)?;
    Ok(())
}

#[cfg(any(freedesktop, target_os = "windows"))]
pub fn delete(args: &DeleteArgs) -> Result<()> {
    let items = select_items(args.finder.cmd(), args.all)?;

    if !items.is_empty() && confirm_or_yes("Delete above items?") {
        trash::os_limited::purge_all(items)?;
    }

    Ok(())
}

#[cfg(any(freedesktop, target_os = "windows"))]
pub fn clean(args: &CleanArgs) -> Result<()> {
    use crate::utils::time;

    let mut items = items(args.all)?;
    time::retain_during(&mut items, args.since.as_deref(), args.until.as_deref())?;
    if items.is_empty() {
        return Ok(());
    }

    render(&mut BufWriter::new(stdout()), &items)?;
    if !confirm_or_no("Clean above items?") {
        return Ok(());
    }

    trash::os_limited::purge_all(items)?;

    Ok(())
}
