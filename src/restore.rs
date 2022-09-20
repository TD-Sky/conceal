use crate::utils::confirm;
use anyhow::{Context, Result};
use chrono::{Local, TimeZone};
use std::{
    env,
    io::Write,
    process::{Command, Stdio},
};
use trash::os_limited::{list, restore_all};

pub fn restore() -> Result<()> {
    // Users only can restore files discarded under the current directory.
    let pwd = env::current_dir()?;

    let mut items: Vec<_> = list()?
        .into_iter()
        .filter(|item| item.original_parent.starts_with(&pwd))
        .collect();

    items.sort_by_key(|item| item.time_deleted);

    let options = items
        .iter()
        .enumerate()
        .rev() // For getting closest to the files discarded recently.
        .map(|(i, item)| {
            let src = item.original_path();
            // Having filter first,
            // the `unwrap` would never fail.
            let src = src.strip_prefix(&pwd).unwrap().to_string_lossy();

            let time = Local.timestamp(item.time_deleted, 0);
            let time = time.format("%Y-%m-%d %H:%M:%S");

            format!("{i} {time} {src}")
        })
        .collect::<Vec<_>>()
        .join("\n"); // Tail '\n' is forbidden.

    // conceal list current directory trash
    // | sk --multi
    // | conceal restore selected trash
    let mut skim = Command::new("sk")
        .arg("-m")
        .arg("--no-sort")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    skim.stdin
        .as_mut()
        .context("Unable to acquire stdin of skim")?
        .write_all(options.as_bytes())?;

    let selected = String::from_utf8(skim.wait_with_output()?.stdout)?;

    // Select nothing,
    if selected == "" {
        // Keep silent.
        return Ok(());
    }

    let selected: Vec<usize> = selected
        .trim_end() // Tail '\n' is forbidden for `split` here.
        .split('\n')
        .map(|item| {
            item.splitn(2, ' ')
                .next()
                .unwrap() // Never fail
                .parse()
                .unwrap() // Never fail
        })
        .rev() // The selected items is inverse by the index, rearrange them.
        .collect();

    // Reserve the selected items in `items`.
    let len = selected.len();
    let mut low = 0;
    for high in selected {
        items.swap(low, high);
        low += 1;
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

    Ok(match confirm(prompt) {
        true => restore_all(items)?,
        false => (),
    })
}
