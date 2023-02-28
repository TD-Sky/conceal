use crate::utils::confirm;
use crate::utils::time::{LocalDateTimeHelper, UnixTimestampToString};
use crate::utils::valid_part;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use std::env;
use std::io::Write;
use std::num::ParseIntError;
use std::process::{Command, Stdio};
use trash::os_limited::list;
use trash::os_limited::restore_all;

pub fn restore() -> Result<()> {
    let local_date_time_helper = LocalDateTimeHelper::default();

    // Users only can restore files discarded under the current directory.
    let pwd = env::current_dir()?;

    let mut items: Vec<_> = list()?
        .into_iter()
        .filter(|item| item.original_parent.starts_with(&pwd))
        .collect();
    let width = (items.len() as f64).log10().ceil() as usize;

    items.sort_by_key(|item| item.time_deleted);

    let options = items
        .iter()
        .enumerate()
        .rev() // For getting closest to the files discarded recently.
        .map(|(i, item)| {
            let src = item.original_path();
            // Having filtered before,
            // the `unwrap` would never fail.
            let src = src.strip_prefix(&pwd).unwrap().to_string_lossy();
            let time = item.time_deleted.to_string(&local_date_time_helper);

            format!(
                "\x1b[32;1m{i:>width$}\x1b[0m \
                 \x1b[94m{time}\x1b[0m \
                {src}"
            )
        })
        .collect::<Vec<_>>()
        .join("\n"); // Tail '\n' is forbidden.

    // conceal list current directory trash
    // | sk --multi --ansi
    // | conceal restore selected trash
    let mut skim = Command::new("sk")
        .args(["-m", "--ansi"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| anyhow!("`skim` not found"))?;

    skim.stdin
        .as_mut()
        .context("Unable to acquire stdin of skim")?
        .write_all(options.as_bytes())?;

    let selected = String::from_utf8(skim.wait_with_output()?.stdout)?;

    // Select nothing,
    if selected.is_empty() {
        // Keep silent.
        return Ok(());
    }

    let selected = selected
        .trim_end() // Tail '\n' is forbidden for `split` here.
        .split('\n')
        .map(|item| {
            // There're white spaces at the left side of index
            // because of right aligned.
            item.trim_start()
                .split(' ')
                .next()
                .expect("Fail to split out the index")
                .parse()
        })
        .rev() // The selected items is inverse by the index, rearrange them.
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    // Reserve the selected items in `items`.
    let len = selected.len();
    for (low, high) in selected.into_iter().enumerate() {
        items.swap(low, high);
    }
    items.truncate(len);

    // Ask the users if they want to restore.
    let prompt: String = items
        .iter()
        .map(|item| {
            let src = item.original_path();
            let src = src.to_string_lossy();

            format!("{src}\n")
        })
        .collect::<String>()
        + "\nRestore ? (y/n) ";

    if confirm(prompt) {
        use trash::Error::*;

        restore_all(valid_part(items)).map_err(|e| match e {
            // Before removal on the filesystem
            RestoreTwins { path, .. } => {
                anyhow!("Restoring multiple items to {path:?} is not allowed")
            }

            // During removal on the filesystem
            RestoreCollision { path, .. } => anyhow!("{path:?} has already existed"),

            _ => unreachable!(),
        })?;
    }

    Ok(())
}
