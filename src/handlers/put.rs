use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use std::path::Path;
use trash::delete_all;

pub fn put(items: &[impl AsRef<Path>]) -> Result<()> {
    // Function `delete_all` wouldn't fail even if no file is specified.
    // But this doesn't meet our expectation.
    if items.is_empty() {
        bail!("Please specify the files to trash");
    }

    use trash::Error::*;
    delete_all(items).map_err(|e| match e {
        Unknown { description } => anyhow!("{description}"),

        TargetedRoot => anyhow!("You cannot remove root directory!"),

        CouldNotAccess { .. } => anyhow!("Current working directory lost"),

        _ => unreachable!(),
    })
}
