use anyhow::{bail, Result};
use std::path::PathBuf;
use trash::delete_all;

pub fn put(items: Vec<PathBuf>) -> Result<()> {
    // Function `delete_all` wouldn't fail even if no file is specified.
    // But this doesn't meet our expectation.
    if items.is_empty() {
        bail!("Please specify the files to trash")
    }

    Ok(delete_all(items)?)
}
