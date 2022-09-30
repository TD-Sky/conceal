use crate::utils::{confirm, valid_part};
use anyhow::{anyhow, Result};
use std::env;
use trash::{
    os_limited::{list, purge_all},
    Error::Unknown,
};

pub fn empty() -> Result<()> {
    let trash_bin = env::var("HOME")? + "/.local/share/Trash";

    if confirm(format!("Empty {trash_bin} ? (y/n) ")) {
        purge_all(valid_part(list()?)).map_err(|unknown| {
            // It must be `Unknown`
            if let Unknown { description } = unknown {
                anyhow!("{description}")
            } else {
                unreachable!()
            }
        })?;
    }

    Ok(())
}
