use anyhow::{anyhow, bail, Result};
use std::path::PathBuf;
use trash::{delete_all, Error::*};

pub fn put(items: Vec<PathBuf>) -> Result<()> {
    // Function `delete_all` wouldn't fail even if no file is specified.
    // But this doesn't meet our expectation.
    if items.is_empty() {
        bail!("Please specify the files to trash");
    }

    delete_all(items).map_err(|e| match e {
        Unknown { description } => anyhow!("{description}"),

        TargetedRoot => anyhow!("You cannot remove root directory!"),

        CouldNotAccess { .. } => anyhow!("Current working directory lost"),

        _ => unreachable!(),
    })
}
