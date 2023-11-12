use std::env;
use std::fmt::Write as _;
use std::io::Write;
use std::process::{Command, Stdio};

use owo_colors::OwoColorize;
use trash::os_limited::list;
use trash::os_limited::restore_all;

use crate::error::{Error, Result};
use crate::util::confirm;
use crate::util::time::{self, local_datetime};

pub fn restore(finder: &'static str) -> Result<()> {
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
            let src = src
                .strip_prefix(&pwd)
                .unwrap() // will definitely succeed
                .to_string_lossy();
            let time = local_datetime(item.time_deleted).format(time::FORMAT);

            format!(
                "{i:<width$} {time} {src}",
                i = i.bright_purple().bold(),
                time = time.bright_yellow(),
            )
        })
        .collect::<Vec<String>>()
        .join("\n"); // Tail '\n' is forbidden.

    // conceal list current directory trash
    // | <finder> --multi --ansi
    // | conceal restore selected trash
    let mut finder = Command::new(finder)
        .args(["-m", "--ansi", "--cycle"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| Error::FinderNotFound(finder))?;

    finder
        .stdin
        .as_mut()
        .unwrap()
        .write_all(options.as_bytes())?;

    // Linux shouldn't have UTF-8 problems
    let selected = String::from_utf8(finder.wait_with_output()?.stdout).unwrap();

    // Select nothing,
    if selected.is_empty() {
        // Keep silent.
        return Ok(());
    }

    let selected: Vec<usize> = selected
        .trim_end() // Tail '\n' is forbidden for `split` here.
        .split('\n')
        .map(|item| {
            // There're white spaces at the left side of index
            // because of right aligned.
            item.trim_start()
                .split_once(' ') // white space between index and date time
                .and_then(|(index, _)| index.parse().ok())
                .unwrap() // will definitely succeed
        })
        .rev() // The selected items is inverse by the index, rearrange them.
        .collect();

    // Reserve the selected items in `items`.
    let len = selected.len();
    for (low, high) in selected.into_iter().enumerate() {
        items.swap(low, high);
    }
    items.truncate(len);

    // Ask the users if they want to restore.
    let prompt = items.iter().fold(String::new(), |mut s, item| {
        let src = item.original_path();
        let src = src.to_string_lossy();
        let _ = writeln!(s, "{src}");
        s
    }) + "\nRestore ? (y/n) ";

    if confirm(&prompt) {
        restore_all(items)?;
    }

    Ok(())
}
