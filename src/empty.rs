use crate::utils::confirm;
use anyhow::Result;
use std::env;
use trash::os_limited::{list, purge_all};

pub fn empty() -> Result<()> {
    let trash_bin = env::var("HOME")? + "/.local/share/Trash";

    Ok(match confirm(format!("Empty {trash_bin}")) {
        true => purge_all(list()?)?,
        false => (),
    })
}
