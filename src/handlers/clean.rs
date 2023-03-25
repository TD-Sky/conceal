use crate::utils::confirm;
use crate::utils::valid_part;
use anyhow::anyhow;
use anyhow::Result;
use std::env;
use trash::os_limited::list;
use trash::os_limited::purge_all;

pub fn clean() -> Result<()> {
    let trash_bin = env::var("HOME")? + "/.local/share/Trash";

    use trash::Error::Unknown;
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
