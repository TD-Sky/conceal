use std::path::Path;

use trash::delete_all;

use crate::error::Result;

pub fn put(items: &[impl AsRef<Path>]) -> Result<()> {
    // Function `delete_all` wouldn't fail even if no file is specified.
    // But this doesn't meet our expectation.
    if items.is_empty() {
        return Err("Please specify the files to trash".into());
    }

    delete_all(items)?;

    Ok(())
}
